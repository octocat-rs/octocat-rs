use std::sync::Arc;

use async_trait::async_trait;

use github_rest::model::{
    commits::events::CommitCommentEvent,
    issues::events::{IssueCommentEvent, IssueEvent},
    pull_requests::events::{PullRequestEvent, PullRequestReviewCommentEvent, PullRequestReviewEvent},
    releases::events::{CreateEvent, DeleteEvent, ReleaseEvent},
    repositories::{
        events::{ForkEvent, PushEvent, StarEvent},
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

    async fn message(&self, message: Self::Message);

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
