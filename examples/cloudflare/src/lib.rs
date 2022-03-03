use std::sync::Arc;

use async_trait::async_trait;
use worker::{console_log, event, Env, Request, Response, Router};

use octocat_rs::{
    handler::EventHandler,
    rest::model::{misc::events::MarketplacePurchaseEvent, repositories::events::PushEvent},
    Client, ClientBuilder, Command,
};

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> worker::Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();
    let r = router
        .post_async("/payload", |mut req, _| async {
            // TODO: Don't construct this every time
            let client = ClientBuilder::new().event_handler(Handler {}).build().unwrap();
            client.handle(req).await;

            Response::empty()
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        });

    r.run(req, env).await
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
                console_log!("==> Message received: {s}");
            }
        }
    }

    async fn commit_event(
        &self,
        _github_client: Arc<Self::GitHubClient>,
        _commit: PushEvent,
    ) -> Command<Self::Message> {
        console_log!("Commit pushed!");

        Command::perform(async { "Computation finished" }, Message::Stuff)
    }
}
