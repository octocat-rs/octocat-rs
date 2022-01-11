use anyhow::Result;
use async_trait::async_trait;
use github_rest::{model::Commit, Requester};

use octocat_rs::{client::GitHubClient, handler::EventHandler, ClientBuilder, Command};

#[tokio::main]
async fn main() -> Result<()> {
    #[derive(Debug)]
    struct Handler {}

    #[async_trait]
    impl EventHandler for Handler {
        type Message = ();

        fn webhook_port(&self) -> u32 {
            2022
        }

        async fn commit_pushed(
            &self,
            _http_client: &'static (impl Requester + Sync),
            _commit: &'static Commit,
        ) -> Command<Self::Message> {
            println!("Commit pushed!");
            Command::none()
        }
    }

    ClientBuilder::new().event_handler(Handler {}).build()?.start().await
}
