use std::sync::Arc;

use async_trait::async_trait;

use github_rest::model::{
    apps::events::{AppAuthorizationEvent, InstallationEvent, InstallationRepositoriesEvent},
    commits::events::CommitCommentEvent,
    issues::events::{IssueCommentEvent, IssueEvent},
    pull_requests::events::{PullRequestEvent, PullRequestReviewCommentEvent, PullRequestReviewEvent},
    releases::events::{CreateEvent, DeleteEvent, ReleaseEvent},
    repositories::{
        events::{
            DeployKeyEvent, ForkEvent, MemberEvent, MilestoneEvent, PublicEvent, PushEvent, RepositoryDispatchEvent,
            RepositoryEvent, RepositoryImportEvent, RepositoryVulnerabilityAlertEvent, SecretScanningAlertEvent,
            StarEvent, WatchEvent,
        },
        workflows::events::{CheckRunEvent, WorkflowJobEvent, WorkflowRunEvent},
    },
};

use crate::{client::GitHubClient, github::command::Command, Client};

/// An event handler that is used in all clients. For end users, this is passed
/// to a [`ClientBuilder`] instance when creating the client in your main
/// function.
///
/// [`ClientBuilder`]: crate::github::ClientBuilder
#[async_trait]
#[allow(unused_variables)]
pub trait EventHandler {
    type Message: std::fmt::Debug + Send;
    type GitHubClient: GitHubClient + Send + Sync;

    /// Utility function for getting the port used by the webhook.
    fn webhook_port(&self) -> u16 {
        8080
    }

    /// The route at which the listener should listen for payloads from GitHub.
    fn route(&self) -> &'static str {
        "/payload"
    }

    async fn message(&self, message: Self::Message) {
        {}
    }

    /// Someone revokes their authorization of a GitHub App
    async fn app_authorization_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        app_event: AppAuthorizationEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a GitHub App installation
    async fn installation_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        installation_event: InstallationEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    async fn installation_repositories_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        installation_repositories_event: InstallationRepositoriesEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Commit pushed to a repository.
    async fn commit_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        push_event: PushEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Comment added to a repository commit.
    async fn commit_comment_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        commit_comment_event: CommitCommentEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Release created
    async fn release_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        release_event: ReleaseEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Repository-related activity
    async fn repository_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        repository_event: RepositoryEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// GitHub App sends a POST request to the "[Create a repository dispatch event](https://docs.github.com/en/rest/reference/repos#create-a-repository-dispatch-event)" endpoint.
    async fn repository_dispatch_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        repository_event: RepositoryDispatchEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a repository being imported to GitHub
    async fn repository_import_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        repository_event: RepositoryImportEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to security vulnerability alerts in a repository.
    async fn repository_vulnerability_alert(
        &self,
        github_client: Arc<Self::GitHubClient>,
        repository_vulnerability_alert: RepositoryVulnerabilityAlertEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to secret scanning alerts in a repository.
    async fn secret_scanning_alert(
        &self,
        github_client: Arc<Self::GitHubClient>,
        secret_scanning_alert: SecretScanningAlertEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Deploy key added or removed from a repository.
    async fn deploy_key_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        deploy_key_event: DeployKeyEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Event related to repository collaborators
    async fn member_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        member_event: MemberEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Repository made public.
    async fn public_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        public_event: PublicEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Event related to repository milestones.
    async fn milestone_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        milestone_event: MilestoneEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Git branch or tag created
    async fn tag_created(&self, github_client: Arc<Self::GitHubClient>, event: CreateEvent) -> Command<Self::Message> {
        Command::none()
    }

    /// Git branch or tag deleted
    async fn tag_deleted(&self, github_client: Arc<Self::GitHubClient>, event: DeleteEvent) -> Command<Self::Message> {
        Command::none()
    }

    /// Repository receives a star
    async fn star_event(&self, github_client: Arc<Self::GitHubClient>, event: StarEvent) -> Command<Self::Message> {
        Command::none()
    }

    /// Someone begins watching a repository
    async fn watch_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        watch_event: WatchEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Repository is forked
    async fn repository_forked(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: ForkEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    async fn pull_request_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: PullRequestEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    async fn pull_request_review_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: PullRequestReviewEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    async fn pull_request_review_comment_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: PullRequestReviewCommentEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    async fn workflow_run(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: WorkflowRunEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    async fn workflow_job(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: WorkflowJobEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    async fn check_run(&self, github_client: Arc<Self::GitHubClient>, event: CheckRunEvent) -> Command<Self::Message> {
        Command::none()
    }

    async fn issue_event(&self, github_client: Arc<Self::GitHubClient>, event: IssueEvent) -> Command<Self::Message> {
        Command::none()
    }

    async fn issue_comment_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: IssueCommentEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }
}

#[derive(Debug)]
pub struct DefaultEventHandler;

#[async_trait]
impl EventHandler for DefaultEventHandler {
    type Message = ();
    type GitHubClient = Client<Self>;

    async fn message(&self, _message: Self::Message) {}
}

impl DefaultEventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultEventHandler {
    fn default() -> Self {
        Self::new()
    }
}
