use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;
use warp::Filter;

use github_rest::{
    methods::{api_info, get_commits, get_issues, get_pulls, prelude::GetResponse, zen},
    model::{
        apps::events::{AppAuthorizationEvent, InstallationEvent, InstallationRepositoriesEvent},
        commits::{
            events::{CommitCommentEvent, StatusEvent},
            Commits,
        },
        discussions::events::{DiscussionCommentEvent, DiscussionEvent},
        event_types::EventTypes,
        issues::{
            events::{IssueCommentEvent, IssueEvent},
            Issues,
        },
        misc::events::{DeploymentEvent, DeploymentStatusEvent, LabelEvent},
        organizations::events::{MembershipEvent, OrgBlockEvent, OrganizationEvent, TeamEvent},
        pull_requests::{
            events::{PullRequestEvent, PullRequestReviewCommentEvent, PullRequestReviewEvent},
            Pulls,
        },
        releases::events::{CreateEvent, DeleteEvent, ReleaseEvent},
        repositories::{
            events::{
                BranchProtectionRuleEvent, CodeScanningAlertEvent, DeployKeyEvent, ForkEvent, MemberEvent,
                MilestoneEvent, ProjectEvent, PublicEvent, PushEvent, RepositoryDispatchEvent, RepositoryEvent,
                RepositoryImportEvent, RepositoryVulnerabilityAlertEvent, SecretScanningAlertEvent, StarEvent,
                TeamAddEvent, WatchEvent,
            },
            workflows::events::{
                CheckRunEvent, CheckSuiteEvent, PageBuildEvent, WorkflowDispatchEvent, WorkflowJobEvent,
                WorkflowRunEvent,
            },
        },
    },
    GithubRestError, Requester,
};

use crate::{
    github::{handler::EventHandler, util::Authorization, DefaultEventHandler, HttpClient},
    Command,
};

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
    pub(crate) http_client: HttpClient,
}

#[async_trait]
impl<T> GitHubClient for Client<T>
where
    T: Debug + EventHandler<GitHubClient = Client<T>> + Send + Sync,
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
    T: Debug + EventHandler<GitHubClient = Client<T>> + Send + Sync + 'static,
{
    pub async fn start(self) {
        let _ = self.run().await.expect("Starting application: User-defined code");

        let self_arc = Arc::new(self);
        let thread_self = self_arc.clone();
        let thread_self_2 = self_arc.clone();
        let (tx, mut rx) = mpsc::channel(32);

        let event_type = warp::post()
            .and(warp::path(self_arc.handler.route()))
            .and(warp::header::<EventTypes>("X-GitHub-Event"))
            .and(warp::body::content_length_limit(self_arc.max_payload_size)) // 8Kb
            .and(warp::body::json());

        let routes = event_type.map(move |ev: EventTypes, body: serde_json::Value| {
            let mut user_cmd = Command::none();

            macro_rules! event_push {
                ($f:ident, $t:ty) => {
                    user_cmd = thread_self
                        .event_handler()
                        .$f(
                            thread_self.clone(),
                            serde_json::from_str::<$t>(body.to_string().as_str()).expect("Failed to parse json"),
                        )
                        .await
                };
            }

            dbg!(&ev);
            dbg!(&body.to_string());

            let ev_h = async {
                match ev {
                    EventTypes::Push => {
                        event_push!(commit_event, PushEvent);
                    }
                    EventTypes::GithubAppAuthorization => {
                        event_push!(app_authorization_event, AppAuthorizationEvent);
                    }
                    EventTypes::Installation => {
                        event_push!(installation_event, InstallationEvent);
                    }
                    EventTypes::InstallationRepositories => {
                        event_push!(installation_repositories_event, InstallationRepositoriesEvent);
                    }
                    EventTypes::DeployKey => {
                        event_push!(deploy_key_event, DeployKeyEvent);
                    }
                    EventTypes::Gollum => {
                        // TODO: Remove this mock code; it's only here for testing purposes.
                        user_cmd = thread_self
                            .event_handler()
                            .commit_event(thread_self.clone(), Default::default())
                            .await;
                    }
                    EventTypes::Member => {
                        event_push!(member_event, MemberEvent);
                    }
                    EventTypes::Milestone => {
                        event_push!(milestone_event, MilestoneEvent);
                    }
                    EventTypes::Public => {
                        event_push!(public_event, PublicEvent);
                    }
                    EventTypes::Release => {
                        event_push!(release_event, ReleaseEvent);
                    }
                    EventTypes::Repository => {
                        event_push!(repository_event, RepositoryEvent);
                    }
                    EventTypes::RepositoryDispatch => {
                        event_push!(repository_dispatch_event, RepositoryDispatchEvent);
                    }
                    EventTypes::RepositoryImport => {
                        event_push!(repository_import_event, RepositoryImportEvent);
                    }
                    EventTypes::RepositoryVulnerabilityAlert => {
                        event_push!(repository_vulnerability_alert, RepositoryVulnerabilityAlertEvent);
                    }
                    EventTypes::SecretScanningAlert => {
                        event_push!(secret_scanning_alert, SecretScanningAlertEvent);
                    }
                    EventTypes::SecurityAdvisory => {} // TODO: Sort this out <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#security_advisory>
                    EventTypes::Star => {
                        event_push!(star_event, StarEvent);
                    }
                    EventTypes::Watch => {
                        event_push!(watch_event, WatchEvent);
                    }
                    EventTypes::PullRequest => {
                        event_push!(pull_request_event, PullRequestEvent);
                    }
                    EventTypes::PullRequestReview => {
                        event_push!(pull_request_review_event, PullRequestReviewEvent);
                    }
                    EventTypes::PullRequestReviewComment => {
                        event_push!(pull_request_review_comment_event, PullRequestReviewCommentEvent);
                    }
                    EventTypes::CommitComment => {
                        event_push!(commit_comment_event, CommitCommentEvent);
                    }
                    EventTypes::Status => {
                        event_push!(status_event, StatusEvent);
                    }
                    EventTypes::IssueComment => {
                        event_push!(issue_comment_event, IssueCommentEvent);
                    }
                    EventTypes::Issues => {
                        event_push!(issue_event, IssueEvent);
                    }
                    EventTypes::Label => {
                        event_push!(label_event, LabelEvent);
                    }
                    EventTypes::Discussion => {
                        event_push!(discussion_event, DiscussionEvent);
                    }
                    EventTypes::DiscussionComment => {
                        event_push!(discussion_comment_event, DiscussionCommentEvent);
                    }
                    EventTypes::BranchProtectionRule => {
                        event_push!(branch_protection_rule_event, BranchProtectionRuleEvent);
                    }
                    EventTypes::Create => {
                        event_push!(tag_created, CreateEvent);
                    }
                    EventTypes::Delete => {
                        event_push!(tag_deleted, DeleteEvent);
                    }
                    EventTypes::Fork => {
                        event_push!(repository_forked, ForkEvent);
                    }
                    EventTypes::CheckRun => {
                        event_push!(check_run, CheckRunEvent);
                    }
                    EventTypes::CheckSuite => {
                        event_push!(check_suite_event, CheckSuiteEvent);
                    }
                    EventTypes::CodeScanningAlert => {
                        event_push!(code_scanning_alert_event, CodeScanningAlertEvent);
                    }
                    EventTypes::Deployment => {
                        event_push!(deployment_event, DeploymentEvent);
                    }
                    EventTypes::DeploymentStatus => {
                        event_push!(deployment_status_event, DeploymentStatusEvent);
                    }
                    EventTypes::PageBuild => {
                        event_push!(page_build_event, PageBuildEvent);
                    }
                    EventTypes::WorkflowDispatch => {
                        event_push!(workflow_dispatch_event, WorkflowDispatchEvent);
                    }
                    EventTypes::WorkflowJob => {
                        event_push!(workflow_job, WorkflowJobEvent);
                    }
                    EventTypes::WorkflowRun => {
                        event_push!(workflow_run, WorkflowRunEvent);
                    }
                    EventTypes::Membership => {
                        event_push!(membership_event, MembershipEvent);
                    }
                    EventTypes::OrgBlock => {
                        event_push!(org_block_event, OrgBlockEvent);
                    }
                    EventTypes::Organization => {
                        event_push!(organization_event, OrganizationEvent);
                    }
                    EventTypes::Team => {
                        event_push!(team_event, TeamEvent);
                    }
                    EventTypes::TeamAdd => {
                        event_push!(team_add_event, TeamAddEvent);
                    }
                    EventTypes::Project => {
                        event_push!(project_event, ProjectEvent);
                    }
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
            warp::serve(routes).run(([127, 0, 0, 1], self_arc.event_handler().listener_port())),
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
