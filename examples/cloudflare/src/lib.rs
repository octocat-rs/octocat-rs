use std::sync::Arc;

use async_trait::async_trait;
use worker::{console_log, event, Date, Env, Request, Response};

use octocat_rs::{handler::EventHandler, rest::model::repositories::events::PushEvent, Client, ClientBuilder, Command};

mod utils;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> anyhow::Result<Response> {
    log_request(&req);

    ClientBuilder::new()
        .event_handler(Handler {})
        .build()?
        .handle(req)
        .await;

    // This can't fail, so using anyhow::Result is preferred over worker::Result
    Ok(Response::empty().unwrap())
}

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
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
    async fn commit_event(
        &self,
        _github_client: Arc<Self::GitHubClient>,
        _commit: PushEvent,
    ) -> Command<Self::Message> {
        println!("Commit pushed!");

        Command::perform(async { "Computation finished" }, Message::Stuff)
    }
}
