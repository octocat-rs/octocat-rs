use crate::github::{handler::EventHandler, util::Authorization, DefaultEventHandler};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GitHubClient {
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
    authorization: Authorization,
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
    pub fn new(handler: T, auth: Authorization) -> Self {
        Self {
            handler,
            authorization: auth,
        }
    }

    pub fn set_auth(self, auth: Authorization) -> Self {
        Self {
            handler: self.handler,
            authorization: auth,
        }
    }
}

impl Default for Client<DefaultEventHandler> {
    fn default() -> Client<DefaultEventHandler> {
        Client {
            handler: DefaultEventHandler,
            authorization: Authorization::default(),
        }
    }
}
