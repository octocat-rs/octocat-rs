pub use client::*;
pub use client_builder::*;
pub use command::*;
pub use handler::*;
pub use http::*;
pub use util::*;

pub mod client;
pub mod client_builder;
pub mod command;
pub mod handler;
pub mod http;
pub mod util;

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use async_trait::async_trait;

    use crate::github::{
        client::GitHubClient, command::Command, handler::EventHandler, ClientBuilder, DefaultEventHandler,
    };

    #[test]
    fn default_everything() {
        let _client = ClientBuilder::build_unconfigured();
    }

    #[tokio::test]
    async fn standard() -> Result<()> {
        #[derive(Debug)]
        struct Handler {}

        #[async_trait]
        impl EventHandler for Handler {
            type Message = ();

            fn webhook_url(&self) -> Option<&str> {
                Some("https://example.com/hook")
            }

            // Default behavior for all paths
            async fn comment_reaction_received(&self) -> Command<Self::Message> {
                Command::none()
            }
        }

        ClientBuilder::new()
            .event_handler(Handler {})
            .credentials_file("examples/octocat.example.toml")
            .build()
            .unwrap()
            .start()
            .await
    }

    #[tokio::test]
    async fn custom_handler() {
        #[derive(Debug)]
        struct Handler;

        #[async_trait]
        impl EventHandler for Handler {
            type Message = ();

            fn webhook_url(&self) -> Option<&str> {
                None
            }

            async fn comment_reaction_received(&self) -> Command<Self::Message> {
                Command::none()
            }
        }

        let _client = ClientBuilder::new()
            .event_handler(Handler)
            .build()
            .unwrap()
            .start()
            .await;
    }

    #[test]
    fn auth_no_handler() {
        let _client = ClientBuilder::<DefaultEventHandler>::new()
            .personal_auth("", "")
            .build_no_handler();
    }
}
