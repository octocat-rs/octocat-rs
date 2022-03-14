use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use lazy_static::lazy_static;

use github_rest::model::repositories::events::PingEvent;
use octocat_rs::{handler::EventHandler, Client, ClientBuilder, Command};

lazy_static! {
    static ref WEBHOOK_SECRET: String = std::env::var("WEBHOOK_SECRET").unwrap();
}

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

    fn listener_secret(&self) -> &'static [u8] {
        WEBHOOK_SECRET.as_bytes()
    }

    async fn message(&self, message: Self::Message) {
        match message {
            Message::Stuff(s) => {
                println!("==> Message received: {s}");
            }
        }
    }

    async fn ping_event(&self, _github_client: Arc<Self::GitHubClient>, _ping_event: PingEvent) -> Command<Self::Message> {
        println!("Secure webhook created!");

        Command::perform(async { "Computation finished" }, Message::Stuff)
    }
}
