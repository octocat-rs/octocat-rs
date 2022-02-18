pub use client::*;
pub use client_builder::*;
pub use command::*;
pub use handler::*;
pub use http::*;
pub use util::*;

/// Contains the [`GitHubClient`] trait and its default implementation
/// ([`Client`]).
pub mod client;
/// Contains a builder for [`Client`].
pub mod client_builder;
/// Contains the [`Command`] interface used in the event [`handler`].
pub mod command;
/// Contains the [`EventHandler`] trait and its default implementation
/// ([`DefaultEventHandler`]).
pub mod handler;
/// Contains [`HttpClient`].
pub mod http;
/// Miscellaneous utilities including the [`OctocatConfig`] struct and
/// [`Authorization`] enum.
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
        github::{command::Command, handler::EventHandler, ClientBuilder},
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

            fn listener_port(&self) -> u16 {
                8080
            }

            async fn message(&self, _message: Self::Message) {}

            /// Example for how [`Command::perform`] should be used in practice.
            async fn commit_event(
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
    /*
    #[test]
    fn no_auth_no_handler() {
        // TODO: Figure out what on earth is wrong with this
        let _client = ClientBuilder::<DefaultEventHandler>::new().build_no_handler();
    }*/
}
