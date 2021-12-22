use anyhow::Result;
use async_trait::async_trait;

use crate::github::{handler::EventHandler, util::Authorization, DefaultEventHandler, HttpClient};

#[async_trait]
pub trait GitHubClient {
    /// The code that the implementer wants to be run at startup.
    async fn run(&self) -> Result<()>;

    async fn start(&self) -> Result<()> {
        // TODO: Runtime
        self.run().await
    }
}

// TODO: HTTP client, Client trait, method impls
/// Where the magic happens.
#[allow(dead_code)]
pub struct Client<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    handler: T,
    http_client: HttpClient,
}

#[async_trait]
impl<T> GitHubClient for Client<T>
where
    T: std::fmt::Debug + EventHandler + Send + Sync,
{
    async fn run(&self) -> Result<()> {
        Ok(())
    }
}

impl<T> Client<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    /// Creates a new [`Client`].
    pub fn new(handler: T, auth: Authorization) -> Self {
        Self {
            handler,
            http_client: HttpClient::new(auth),
        }
    }

    /// Updates the authorization parameter in the current [`Client`] instance.
    pub fn set_auth(self, auth: Authorization) -> Self {
        Self {
            handler: self.handler,
            http_client: HttpClient::new(auth),
        }
    }
}

impl Default for Client<DefaultEventHandler> {
    fn default() -> Client<DefaultEventHandler> {
        Client {
            handler: DefaultEventHandler,
            http_client: HttpClient::new(Authorization::default()),
        }
    }
}
