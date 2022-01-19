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
    use std::sync::Arc;

    use async_trait::async_trait;
    use futures::FutureExt;

    use github_rest::{
        model::{commits::comments::CommitComment, repositories::events::PushEvent},
        GithubRestError,
    };

    use crate::{
        client::GitHubClient,
        github::{command::Command, handler::EventHandler, ClientBuilder, DefaultEventHandler},
        Client,
    };

    #[test]
    fn default_everything() {
        let _client = ClientBuilder::build_unconfigured();
    }

    #[tokio::test]
    async fn standard() {
        #[derive(Debug)]
        struct Handler {}

        #[derive(Debug)]
        enum Message {
            CommentPosted(std::result::Result<CommitComment, GithubRestError>),
        }

        #[async_trait]
        impl EventHandler for Handler {
            type Message = Message;
            type GitHubClient = Client<Self>;

            fn webhook_port(&self) -> u16 {
                8080
            }

            /// Example for how [`Command::perform`] should be used in practice.
            async fn commit_pushed(
                &self,
                github_client: Arc<Self::GitHubClient>,
                commit: PushEvent,
            ) -> Command<Self::Message> {
                let task = tokio::spawn(async move {
                    commit
                        .add_comment(github_client.http_client_arc(), "".to_owned(), None, None)
                        .await
                });

                Command::perform(task.map(|res| res.unwrap()), Message::CommentPosted)
            }
        }

        ClientBuilder::new()
            .event_handler(Handler {})
            .credentials_file("../examples/octocat.example.toml")
            .build()
            .unwrap();
    }

    #[test]
    fn no_auth_no_handler() {
        let _client = ClientBuilder::<DefaultEventHandler>::new().build_no_handler();
    }
}
