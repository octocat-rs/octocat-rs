use std::fmt::Debug;

use crate::github;
use anyhow::Result;
use async_trait::async_trait;

// TODO: Shared request methods
/// A trait to be implemented by you, the user.
#[async_trait]
pub trait GitHubApplication {
    /// The code that is run when your application starts. Called by [`start`].
    ///
    /// [`start`]: GitHubApplication::start
    async fn run(&self) -> Result<()>;

    // TODO: Settings interface
    async fn start(&self) -> Result<()> {
        // TODO: Proper logging etc
        self.run().await
    }
}

/// An event handler to be implemented by the user.
#[async_trait]
pub trait Handler {
    type Message: Debug + Send;

    // TODO: Work out other means of getting updates
    fn webhook_url(&self) -> &str;

    // Example
    async fn comment_reaction_received(&self)
        -> github::Command<Self::Message>;
}
