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
    use async_trait::async_trait;
    use github_rest::{
        model::{nested::CommitComment, Commit},
        GithubRestError, Requester,
    };

    use crate::github::{command::Command, handler::EventHandler, ClientBuilder, DefaultEventHandler};

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

            fn webhook_port(&self) -> u32 {
                8080
            }

            /// Example for how [`Command::perform`] should be used in practice.
            async fn commit_pushed(
                &self,
                http_client: &'static (impl Requester + Sync),
                commit: &'static Commit,
            ) -> Command<Self::Message> {
                Command::perform(
                    commit.add_comment(http_client, "".to_owned(), None, None),
                    Message::CommentPosted,
                )
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
