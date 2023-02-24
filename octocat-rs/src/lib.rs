#![feature(str_split_remainder)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::str_to_string)]
#![deny(rust_2018_idioms)]

//! A GitHub API client written in Rust.
//!
//! Getting started? Take a look at the [examples](https://github.com/octocat-rs/octocat-rs/tree/main/examples) folder in the project repository!

pub use github::*;

pub mod github;

pub use github_api_octocat as api;
pub use github_rest as rest;

#[cfg(all(feature = "native", feature = "workers"))]
compile_error!("feature \"native\" and feature \"workers\" cannot be enabled at the same time");

#[cfg(test)]
mod tests {
    use crate::{github::ClientBuilder, DefaultEventHandler};

    #[test]
    fn default_everything() {
        let _client = ClientBuilder::build_unconfigured();
    }

    #[cfg(feature = "native")]
    #[tokio::test]
    async fn standard() {
        use crate::{handler::EventHandler, Client, Command};
        use async_trait::async_trait;
        use futures::FutureExt;
        use github_rest::{
            model::{commits::comments::CommitComment, repositories::events::PushEvent},
            GithubRestError,
        };
        use std::sync::Arc;

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

            fn listener_port(&self) -> u16 {
                8080
            }

            #[cfg(feature = "secrets")]
            fn listener_secret(&self) -> &'static [u8] {
                "".as_bytes()
            }

            async fn message(&self, _message: Self::Message) {}

            /// Example for how [`Command::perform`] should be used in practice.
            async fn push_event(
                &self,
                github_client: Arc<Self::GitHubClient>,
                commit: PushEvent,
            ) -> Command<Self::Message> {
                let task = tokio::spawn(async move {
                    commit
                        .add_comment_to_commit(&*github_client, "".to_owned(), None, None)
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
