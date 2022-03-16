use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use github_rest::model::repositories::events::PushEvent;
use octocat_rs::{handler::EventHandler, Client, ClientBuilder, Command};

#[tokio::main]
async fn main() -> Result<()> {
    ClientBuilder::new().event_handler(Handler {}).build()?.start().await;

    Ok(())
}

#[derive(Debug)]
struct Handler {}

#[derive(Debug)]
enum Message {
    Stuff(&'static str),
}

#[async_trait]
impl EventHandler for Handler {
    type Message = Message;
    type GitHubClient = Client<Self>;

    fn listener_port(&self) -> u16 {
        2022
    }

    async fn message(&self, message: Self::Message) {
        match message {
            Message::Stuff(s) => {
                println!("==> Message received: {s}");
            }
        }
    }

    async fn push_event(
        &self,
        _github_client: Arc<Self::GitHubClient>,
        _push_event: PushEvent,
    ) -> Command<Self::Message> {
        println!("Commit pushed!");

        Command::perform(async { "Computation finished" }, Message::Stuff)
    }
}
