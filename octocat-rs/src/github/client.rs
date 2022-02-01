use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;
use warp::Filter;

use crate::Command;
use github_rest::{
    methods::{api_info, get_commits, get_issues, get_pulls, prelude::GetResponse, zen},
    model::{
        apps::events::{AppAuthorizationEvent, InstallationEvent, InstallationRepositoriesEvent},
        commits::{events::CommitCommentEvent, Commits},
        event_types::EventTypes,
        issues::{
            events::{IssueCommentEvent, IssueEvent},
            Issues,
        },
        pull_requests::{
            events::{PullRequestEvent, PullRequestReviewCommentEvent, PullRequestReviewEvent},
            Pulls,
        },
        releases::events::{CreateEvent, DeleteEvent, ReleaseEvent},
        repositories::{
            events::{ForkEvent, PushEvent, StarEvent},
            workflows::events::{CheckRunEvent, WorkflowJobEvent, WorkflowRunEvent},
        },
    },
    GithubRestError, Requester,
};

use crate::github::{handler::EventHandler, util::Authorization, DefaultEventHandler, HttpClient};

#[async_trait]
pub trait GitHubClient {
    type HttpClient: Requester + Send + Sync;
    type EventHandler: EventHandler + Send + Sync;

    /// Code that the implementer wishes to be run *before* the event listener
    /// is started.
    async fn run(&self) -> Result<()>;

    /// Helper function for use in instances where one needs to pass an http
    /// client
    fn http_client(&self) -> &Self::HttpClient;

    fn http_client_arc(&self) -> Arc<&Self::HttpClient> {
        Arc::new(self.http_client())
    }

    fn event_handler(&self) -> &Self::EventHandler;

    /// Helper function to set the maximum payload size. Default is 8 MiB.
    fn payload_size(&self) -> u64 {
        1024 * 8192
    }
    /// Gets all commits from a repository.
    ///
    /// See also: [`get_commits`]
    async fn get_all_commits(&self, owner: String, repo: String) -> std::result::Result<Commits, GithubRestError> {
        get_commits(self.http_client(), owner, repo, None).await
    }

    /// Gets all issues from a repository.
    ///
    /// See also: [`get_issues`]
    async fn get_all_issues(&self, owner: String, repo: String) -> std::result::Result<Issues, GithubRestError> {
        get_issues(self.http_client(), owner, repo, None).await
    }

    /// Gets all pull requests from a repository.
    ///
    /// See also: [`get_pulls`]
    async fn get_all_pulls(&self, owner: String, repo: String) -> std::result::Result<Pulls, GithubRestError> {
        get_pulls(self.http_client(), owner, repo, None).await
    }

    /// Gets all the endpoint categories that the REST API supports.
    ///
    /// See also: [`api_info`]
    async fn get_api_info(&self) -> std::result::Result<GetResponse, GithubRestError> {
        api_info(self.http_client()).await
    }

    /// Gets a random line from the zen of GitHub.
    ///
    /// See also: [`GetZen`]
    ///
    /// [`GetZen`]: github_api::end_points::EndPoints::GetZen
    async fn zen(&self) -> std::result::Result<String, GithubRestError> {
        zen(self.http_client()).await
    }
}

// TODO: Method impls
/// Where the magic happens.
pub struct Client<T>
where
    T: std::fmt::Debug + EventHandler<GitHubClient = Client<T>> + Send + Sync,
{
    handler: T,
    max_payload_size: u64,
    http_client: HttpClient,
}

#[async_trait]
impl<T> GitHubClient for Client<T>
where
    T: std::fmt::Debug + EventHandler<GitHubClient = Client<T>> + Send + Sync,
{
    type HttpClient = HttpClient;
    type EventHandler = T;

    // TODO: User-facing API to set this?
    async fn run(&self) -> Result<()> {
        Ok(())
    }

    fn http_client(&self) -> &Self::HttpClient {
        &self.http_client
    }

    fn event_handler(&self) -> &T {
        &self.handler
    }

    fn payload_size(&self) -> u64 {
        self.max_payload_size
    }
}

impl<T> Client<T>
where
    T: std::fmt::Debug + EventHandler<GitHubClient = Client<T>> + Send + Sync + 'static,
{
    pub async fn start(self) {
        let _ = self.run().await.expect("Starting application: User-defined code");

        let self_arc = Arc::new(self);
        let thread_self = self_arc.clone();
        let thread_self_2 = self_arc.clone();
        let (tx, mut rx) = mpsc::channel(32);

        let event_type = warp::post()
            .and(warp::path("payload"))
            .and(warp::header::<EventTypes>("x-github-event"))
            .and(warp::body::content_length_limit(self_arc.max_payload_size)) // 8Kb
            .and(warp::body::json());

        macro_rules! event_push {
            ($i:ident, $f:ident, $t:ty, $b:ident) => {
                $i = thread_self
                    .event_handler()
                    .$f(
                        thread_self.clone(),
                        serde_json::from_str::<$t>($b.to_string().as_str()).unwrap(),
                    )
                    .await
            };
        }

        let routes = event_type.map(move |ev: EventTypes, body: serde_json::Value| {
            let mut user_cmd = Command::none();

            dbg!(&ev);
            dbg!(&body.to_string());

            let ev_h = async {
                match ev {
                    EventTypes::Push => {
                        event_push!(user_cmd, commit_event, PushEvent, body);
                    }
                    EventTypes::GithubAppAuthorization => {
                        event_push!(user_cmd, app_authorization_event, AppAuthorizationEvent, body);
                    }
                    EventTypes::Installation => {
                        event_push!(user_cmd, installation_event, InstallationEvent, body);
                    }
                    EventTypes::InstallationRepositories => {
                        event_push!(
                            user_cmd,
                            installation_repositories_event,
                            InstallationRepositoriesEvent,
                            body
                        );
                    }
                    EventTypes::DeployKey => {}
                    EventTypes::Gollum => {
                        // TODO: Remove this mock code; it's only here for testing purposes.
                        user_cmd = thread_self
                            .event_handler()
                            .commit_event(thread_self.clone(), Default::default())
                            .await;
                    }
                    EventTypes::Member => {}
                    EventTypes::Milestone => {}
                    EventTypes::Public => {}
                    EventTypes::Release => {
                        event_push!(user_cmd, release_event, ReleaseEvent, body);
                    }
                    EventTypes::Repository => {}
                    EventTypes::RepositoryDispatch => {}
                    EventTypes::RepositoryImport => {}
                    EventTypes::RepositoryVulnerabilityAlert => {}
                    EventTypes::SecretScanningAlert => {}
                    EventTypes::SecurityAdvisory => {}
                    EventTypes::Star => {
                        event_push!(user_cmd, star_event, StarEvent, body);
                    }
                    EventTypes::Watch => {}
                    EventTypes::PullRequest => {
                        event_push!(user_cmd, pull_request_event, PullRequestEvent, body);
                    }
                    EventTypes::PullRequestReview => {
                        event_push!(user_cmd, pull_request_review_event, PullRequestReviewEvent, body);
                    }
                    EventTypes::PullRequestReviewComment => {
                        event_push!(
                            user_cmd,
                            pull_request_review_comment_event,
                            PullRequestReviewCommentEvent,
                            body
                        );
                    }
                    EventTypes::CommitComment => {
                        event_push!(user_cmd, commit_comment_event, CommitCommentEvent, body);
                    }
                    EventTypes::Status => {}
                    EventTypes::IssueComment => {
                        event_push!(user_cmd, issue_comment_event, IssueCommentEvent, body);
                    }
                    EventTypes::Issues => {
                        event_push!(user_cmd, issue_event, IssueEvent, body);
                    }
                    EventTypes::Label => {}
                    EventTypes::Discussion => {}
                    EventTypes::DiscussionComment => {}
                    EventTypes::BranchProtectionRule => {}
                    EventTypes::Create => {
                        event_push!(user_cmd, tag_created, CreateEvent, body);
                    }
                    EventTypes::Delete => {
                        event_push!(user_cmd, tag_deleted, DeleteEvent, body);
                    }
                    EventTypes::Fork => {
                        event_push!(user_cmd, repository_forked, ForkEvent, body);
                    }
                    EventTypes::CheckRun => {
                        event_push!(user_cmd, check_run, CheckRunEvent, body);
                    }
                    EventTypes::CheckSuite => {}
                    EventTypes::CodeScanningAlert => {}
                    EventTypes::Deployment => {}
                    EventTypes::DeploymentStatus => {}
                    EventTypes::PageBuild => {}
                    EventTypes::WorkflowDispatch => {}
                    EventTypes::WorkflowJob => {
                        event_push!(user_cmd, workflow_job, WorkflowJobEvent, body);
                    }
                    EventTypes::WorkflowRun => {
                        event_push!(user_cmd, workflow_run, WorkflowRunEvent, body);
                    }
                    EventTypes::Membership => {}
                    EventTypes::OrgBlock => {}
                    EventTypes::Organization => {}
                    EventTypes::Team => {}
                    EventTypes::TeamAdd => {}
                    EventTypes::Project => {}
                    EventTypes::ProjectCard => {}
                    EventTypes::ProjectColumn => {}
                    EventTypes::MarketplacePurchase => {}
                    EventTypes::Meta => {}
                    EventTypes::Package => {}
                    EventTypes::Ping => {}
                    EventTypes::Sponsorship => {}
                };

                if !user_cmd.is_empty() {
                    let _ = &tx.send(user_cmd).await;
                }
            };

            futures::executor::block_on(ev_h);

            // Needed due to trait bounds
            ""
        });

        let do_cmd_stuff = async {
            while let Some(cmd) = rx.recv().await {
                let mut cmd = cmd.into_futures();

                while let Some(c) = cmd.pop() {
                    thread_self_2.event_handler().message(c.await).await;
                }
            }
        };

        futures::join!(
            warp::serve(routes).run(([127, 0, 0, 1], self_arc.event_handler().webhook_port())),
            do_cmd_stuff
        );
    }

    /// Creates a new [`Client`].
    pub fn new(handler: T, auth: Option<Authorization>, user_agent: Option<String>, payload_size: Option<u64>) -> Self {
        Self {
            handler,
            max_payload_size: payload_size.unwrap_or(1024 * 8192),
            http_client: HttpClient::new(auth, user_agent),
        }
    }

    /// Updates the authorization parameter in the current [`Client`] instance.
    pub fn set_auth(mut self, auth: Authorization) -> Self {
        self.http_client.set_auth(auth);
        self
    }
}

impl Default for Client<DefaultEventHandler> {
    fn default() -> Client<DefaultEventHandler> {
        Client {
            handler: DefaultEventHandler,
            max_payload_size: 1024 * 8192,
            http_client: HttpClient::new(None, None),
        }
    }
}
