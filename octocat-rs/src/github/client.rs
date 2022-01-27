use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use rocket::data::{ByteUnit, ToByteUnit};
use warp::Filter;

use github_rest::{
    methods::{api_info, get_commits, get_issues, get_pulls, prelude::GetResponse, zen},
    model::{
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
    fn payload_size(&self) -> ByteUnit {
        todo!("Complete Warp migration")
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
    max_payload_size: ByteUnit,
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

    fn payload_size(&self) -> ByteUnit {
        self.max_payload_size.mebibytes()
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

        let event_type = warp::post()
            .and(warp::path("payload"))
            .and(warp::header::<EventTypes>("x-github-event"))
            .and(warp::body::content_length_limit(1024 * 8192)) // 8Kb
            .and(warp::body::json());

        let routes = event_type.map(move |ev: EventTypes, body: serde_json::Value| {
            dbg!(&ev);
            dbg!(&body.to_string());
            let a = async {
                match ev {
                    EventTypes::Push => {
                        thread_self
                            .event_handler()
                            .commit_event(
                                thread_self.clone(),
                                serde_json::from_str::<PushEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::GithubAppAuthorization => {}
                    EventTypes::Installation => {}
                    EventTypes::InstallationRepositories => {}
                    EventTypes::DeployKey => {}
                    EventTypes::Gollum => {}
                    EventTypes::Member => {}
                    EventTypes::Milestone => {}
                    EventTypes::Public => {}
                    EventTypes::Release => {
                        thread_self
                            .event_handler()
                            .release_event(
                                thread_self.clone(),
                                serde_json::from_str::<ReleaseEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::Repository => {}
                    EventTypes::RepositoryDispatch => {}
                    EventTypes::RepositoryImport => {}
                    EventTypes::RepositoryVulnerabilityAlert => {}
                    EventTypes::SecretScanningAlert => {}
                    EventTypes::SecurityAdvisory => {}
                    EventTypes::Star => {
                        thread_self
                            .event_handler()
                            .star_event(
                                thread_self.clone(),
                                serde_json::from_str::<StarEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::Watch => {}
                    EventTypes::PullRequest => {
                        thread_self
                            .event_handler()
                            .pull_request_event(
                                thread_self.clone(),
                                serde_json::from_str::<PullRequestEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::PullRequestReview => {
                        thread_self
                            .event_handler()
                            .pull_request_review_event(
                                thread_self.clone(),
                                serde_json::from_str::<PullRequestReviewEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::PullRequestReviewComment => {
                        thread_self
                            .event_handler()
                            .pull_request_review_comment_event(
                                thread_self.clone(),
                                serde_json::from_str::<PullRequestReviewCommentEvent>(body.to_string().as_str())
                                    .unwrap(),
                            )
                            .await;
                    }
                    EventTypes::CommitComment => {
                        thread_self
                            .event_handler()
                            .commit_comment_event(
                                thread_self.clone(),
                                serde_json::from_str::<CommitCommentEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::Status => {}
                    EventTypes::IssueComment => {
                        thread_self
                            .event_handler()
                            .issue_comment_event(
                                thread_self.clone(),
                                serde_json::from_str::<IssueCommentEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::Issues => {
                        thread_self
                            .event_handler()
                            .issue_event(
                                thread_self.clone(),
                                serde_json::from_str::<IssueEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::Label => {}
                    EventTypes::Discussion => {}
                    EventTypes::DiscussionComment => {}
                    EventTypes::BranchProtectionRule => {}
                    EventTypes::Create => {
                        thread_self
                            .event_handler()
                            .tag_created(
                                thread_self.clone(),
                                serde_json::from_str::<CreateEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::Delete => {
                        thread_self
                            .event_handler()
                            .tag_deleted(
                                thread_self.clone(),
                                serde_json::from_str::<DeleteEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::Fork => {
                        thread_self
                            .event_handler()
                            .repository_forked(
                                thread_self.clone(),
                                serde_json::from_str::<ForkEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::CheckRun => {
                        thread_self
                            .event_handler()
                            .check_run(
                                thread_self.clone(),
                                serde_json::from_str::<CheckRunEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::CheckSuite => {}
                    EventTypes::CodeScanningAlert => {}
                    EventTypes::Deployment => {}
                    EventTypes::DeploymentStatus => {}
                    EventTypes::PageBuild => {}
                    EventTypes::WorkflowDispatch => {}
                    EventTypes::WorkflowJob => {
                        thread_self
                            .event_handler()
                            .workflow_job(
                                thread_self.clone(),
                                serde_json::from_str::<WorkflowJobEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
                    }
                    EventTypes::WorkflowRun => {
                        thread_self
                            .event_handler()
                            .workflow_run(
                                thread_self.clone(),
                                serde_json::from_str::<WorkflowRunEvent>(body.to_string().as_str()).unwrap(),
                            )
                            .await;
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
                }
            };

            let _ = futures::executor::block_on(a);

            body.to_string()
        });

        warp::serve(routes)
            .run(([127, 0, 0, 1], self_arc.event_handler().webhook_port()))
            .await;
    }

    /// Creates a new [`Client`].
    pub fn new(handler: T, auth: Option<Authorization>, user_agent: Option<String>, payload_size: Option<u64>) -> Self {
        Self {
            handler,
            max_payload_size: payload_size.unwrap_or(8).mebibytes(),
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
            max_payload_size: 8.mebibytes(),
            http_client: HttpClient::new(None, None),
        }
    }
}
