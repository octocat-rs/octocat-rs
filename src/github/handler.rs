use async_trait::async_trait;

use crate::github::command::Command;

/// An event handler that is used in all clients. For end users, this is passed
/// to a [`ClientBuilder`] instance when creating the client in your main
/// function.
///
/// [`ClientBuilder`]: crate::github::ClientBuilder
#[async_trait]
pub trait EventHandler {
    type Message: std::fmt::Debug + Send;

    /// Utility function for getting the current webhook URL.
    // TODO: Decide on other methods of receiving updates
    fn webhook_url(&self) -> Option<&str>;

    /// Example function for what events may look like
    async fn comment_reaction_received(&self) -> Command<Self::Message>;
}

#[derive(Debug)]
pub struct DefaultEventHandler;

#[async_trait]
impl EventHandler for DefaultEventHandler {
    type Message = ();

    fn webhook_url(&self) -> Option<&str> {
        None
    }

    async fn comment_reaction_received(&self) -> Command<Self::Message> {
        Command::none()
    }
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
