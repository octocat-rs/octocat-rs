pub use client::*;
pub use client_builder::*;
pub use command::*;
pub use handler::*;
pub use util::*;

pub mod client;
pub mod client_builder;
pub mod command;
pub mod handler;
pub mod util;

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::github::{command::Command, handler::EventHandler, ClientBuilder, DefaultEventHandler};

    #[test]
    fn default_everything() {
        let _client = ClientBuilder::build_unconfigured();
    }

    #[test]
    fn custom_handler() {
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

        let _client = ClientBuilder::new().event_handler(Handler).build().unwrap();
    }

    #[test]
    fn auth_no_handler() {
        let _client = ClientBuilder::<DefaultEventHandler>::new()
            .personal_auth("", "")
            .build_no_handler();
    }
}
