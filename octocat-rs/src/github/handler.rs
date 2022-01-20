use std::sync::Arc;

use async_trait::async_trait;

use github_rest::model::{
    commits::{comments::CommitComment, Commit},
    repositories::events::PushEvent,
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

    /// Commit pushed to a repository.
    ///
    /// See also: [`Commit`]
    async fn commit_pushed(&self, github_client: Arc<Self::GitHubClient>, commit: PushEvent) -> Command<Self::Message> {
        Command::none()
    }

    /// Comment added to a repository commit.
    ///
    /// See also: [`Commit`], [`CommitComment`]
    async fn commit_comment_added(
        &self,
        github_client: Arc<Self::GitHubClient>,
        commit: Commit,
        comment: CommitComment,
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
