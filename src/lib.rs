#![feature(future_join, future_poll_fn)]

use std::future::join;
use crate::utils::{EventTypes, ExampleBody};
use std::str::FromStr;
use worker::*;

mod utils;

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

    // Optionally, get more helpful error messages written to the console in the
    // case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name"
    // placeholders, or "*name" catch-alls to match on specific patterns.
    // Alternatively, use `Router::with_data(D)` to provide arbitrary data that
    // will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for
    // handling HTTP functionality and a `RouteContext` which you can use to
    // and get route parameters and Environment bindings like KV Stores, Durable
    // Objects, Secrets, and Variables.
    let r = router
        .post_async("/payload", |mut req, _| async move {
            let headers = req.headers();
            match headers.get("X-GitHub-Event").unwrap() {
                Some(s) => {
                    console_log!("{}", &s);

                    match EventTypes::from_str(s.trim()) {
                        Ok(v) => {
                            match v {
                                EventTypes::Repository => {
                                    let _body = req.json::<ExampleBody>().await;
                                    console_log!("{_body:#?}")
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            console_error!("Error occurred: {e}");
                            return Response::error("Invalid X-GitHub-Event header", 400)
                        },
                    }
                }
                None => return Response::error("Missing X-GitHub-Event header", 400)
            }
            Response::ok("Not a teapot this time")
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        });

    let x = join!(r.run(req, env), async { 1 }).await;

    x.0
}
