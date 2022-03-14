use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use github_rest::model::repositories::events::PushEvent;
use octocat_rs::{handler::EventHandler, Client, ClientBuilder, Command};

#[tokio::main]
async fn main() -> Result<()> {
    ClientBuilder::new()
        .credentials_env_var("USERNAME", "TOKEN")
        .event_handler(Handler {})
        .build()?
        .start()
        .await;

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

    async fn push_event(&self, github_client: Arc<Self::GitHubClient>, push_event: PushEvent) -> Command<Self::Message> {
        println!("Commit pushed!");

        let text = push_event
            .commits
            .iter()
            .map(|x| {
                format!(
                    "`{}`: {}\nModified files: `{}`",
                    x.author.name,
                    x.message,
                    x.modified
                        .iter()
                        .map(|x| { serde_json::to_string(&x).unwrap() })
                        .collect::<Vec<String>>()
                        .join("`, `")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        Command::perform(
            async move {
                push_event
                    .add_comment_to_commit(&*github_client, text, None, None)
                    .await
                    .unwrap();

                "Comment added"
            },
            Message::Stuff,
        )
    }
}
