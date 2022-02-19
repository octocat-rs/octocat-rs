use octocat_rs::worker::{self::*, worker_sys::ResponseInit};
use serde_json::json;
mod utils;
use async_trait::async_trait;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    #[derive(Debug)]
    struct Handler {}

    #[derive(Debug)]
    enum Message {
        Stuff(&'static str),
    }
    #[async_trait::async_trait]
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

    ClientBuilder::new()
        .event_handler(Handler {})
        .build()?
        .handle(req)
        .await;
    Response::empty()
}
