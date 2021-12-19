use async_trait::async_trait;

use crate::github::command::Command;

// TODO: Default handler

/// An event handler that is used in all clients. For end users, this is passed
/// to [`ClientBuilder`] when creating the client in your main function.
///
/// [`ClientBuilder`]: crate::github::ClientBuilder
#[async_trait]
pub trait EventHandler {
    type Message: std::fmt::Debug + Send;

    /// Utility function for getting the current webhook URL.
    fn webhook_url(&self) -> &str;

    /// Example function for what events may look like
    async fn comment_reaction_received(&self) -> Command<Self::Message>;
}
