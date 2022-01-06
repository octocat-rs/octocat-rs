use async_trait::async_trait;
use github_rest::{
    structs::{nested::Comment, Commit},
    Requester,
};

use crate::github::command::Command;

/// An event handler that is used in all clients. For end users, this is passed
/// to a [`ClientBuilder`] instance when creating the client in your main
/// function.
///
/// [`ClientBuilder`]: crate::github::ClientBuilder
#[async_trait]
#[allow(unused_variables)]
pub trait EventHandler {
    type Message: std::fmt::Debug + Send;

    /// Utility function for getting the port used by the webhook.
    // TODO: Webserver for port (rocket) & types
    fn webhook_port(&self) -> u32 {
        8080
    }

    /// The route at which the listener should listen for payloads from GitHub.
    fn route(&self) -> &'static str {
        "/payload"
    }

    /// Commit pushed to a repository.
    ///
    /// See also: [`Commit`]
    async fn commit_pushed(
        &self,
        http_client: &'static (impl Requester + Sync),
        commit: &'static Commit,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Comment added to a repository commit.
    ///
    /// See also: [`Commit`], [`Comment`]
    async fn commit_comment_added(
        &self,
        http_client: &'static (impl Requester + Sync),
        commit: &'static Commit,
        comment: &'static Comment,
    ) -> Command<Self::Message> {
        Command::none()
    }
}

#[derive(Debug)]
pub struct DefaultEventHandler;

#[async_trait]
impl EventHandler for DefaultEventHandler {
    type Message = ();
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
