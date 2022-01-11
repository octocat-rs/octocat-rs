use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use github_rest::{
    methods::{api_info, get_commits, get_issues, get_pulls, prelude::GetResponse, zen},
    model::{Commit, EventTypes, Issues, Pulls},
    GithubRestError, Requester,
};
use rocket::{
    config::LogLevel,
    data::{ByteUnit, Limits, ToByteUnit},
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::github::{handler::EventHandler, util::Authorization, DefaultEventHandler, HttpClient};

// TODO: Fix the issues in github-rest so that this alias is unnecessary
pub type Commits = Vec<Commit>;

#[async_trait]
pub trait GitHubClient {
    type HttpClient: Requester + Send + Sync;
    type EventHandler: EventHandler + Send + Sync;

    /// Code that the implementer wishes to be run *before* the event listener
    /// is started.
    async fn run(&self) -> Result<()>;

    async fn start(&self) -> Result<()> {
        self.run().await.expect("Starting application: User-defined code");

        self.listener().await
    }

    async fn listener(&self) -> Result<()> {
        struct EventType(EventTypes);

        #[async_trait]
        impl<'a> FromRequest<'a> for EventType {
            type Error = EventHeaderError;

            async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
                let val = match request.headers().get_one("x-github-event") {
                    Some(s) => s,
                    None => return Outcome::Failure((Status::BadRequest, EventHeaderError::MissingEventHeader)),
                };

                match EventTypes::from_str(val) {
                    Ok(e) => Outcome::Success(Self(e)),
                    Err(_) => Outcome::Failure((Status::BadRequest, EventHeaderError::UnimplementedEventType)),
                }
            }
        }

        #[derive(Debug, Copy, Clone)]
        enum EventHeaderError {
            UnimplementedEventType,
            MissingEventHeader,
        }

        #[catch(default)]
        fn default(status: Status, req: &Request) -> String {
            format!("{} ({})", status, req.uri())
        }

        // TODO(github-rest): Complete webhook types
        #[post("/", format = "application/json", data = "<payload>")]
        async fn handler(payload: String, ev: EventType) -> Status {
            dbg!(&payload);

            // TODO: Further sorting, delegate to handler instance
            match ev.0 {
                EventTypes::CheckRun => {}
                EventTypes::CheckSuite => {}
                EventTypes::Create => {}
                EventTypes::Delete => {}
                EventTypes::Fork => {}
                EventTypes::IssueComment => {}
                EventTypes::Issues => {}
                EventTypes::Ping => {}
                EventTypes::PullRequest => {}
                EventTypes::Push => {}
                EventTypes::Release => {}
                EventTypes::Star => {}
                EventTypes::Watch => {}
                EventTypes::WorkflowJob => {}
                EventTypes::WorkflowRun => {}
                _ => {}
            }

            Status::Ok
        }

        let figment = rocket::Config::figment()
            .merge(("port", self.event_handler().webhook_port()))
            .merge(("log_level", LogLevel::Critical))
            .merge(("limits", Limits::default().limit("json", self.payload_size())));

        rocket::custom(figment)
            .mount(self.event_handler().route(), routes![handler])
            .register("/", catchers![default])
            .launch()
            .await?;

        Ok(())
    }

    /// Helper function for use in instances where one needs to pass an http
    /// client
    fn http_client(&self) -> &Self::HttpClient;

    fn event_handler(&self) -> &Self::EventHandler;

    /// Helper function to set the maximum payload size. Default is 8 MiB.
    fn payload_size(&self) -> ByteUnit {
        8.mebibytes() // TODO: Figure out why on earth this sets the limit to
                      // 8TiB
    }

    /// Gets all commits from a repository.
    ///
    /// See also: [`get_commits`]
    async fn get_all_commits(&self, owner: String, repo: String) -> std::result::Result<Commits, GithubRestError> {
        get_commits(self.http_client(), owner, repo, None).await
    }

    /// Gets all issues from a repository.
    ///
    /// See also: [`get_issues`]
    async fn get_all_issues(&self, owner: String, repo: String) -> std::result::Result<Issues, GithubRestError> {
        get_issues(self.http_client(), owner, repo, None).await
    }

    /// Gets all pull requests from a repository.
    ///
    /// See also: [`get_pulls`]
    async fn get_all_pulls(&self, owner: String, repo: String) -> std::result::Result<Pulls, GithubRestError> {
        get_pulls(self.http_client(), owner, repo, None).await
    }

    /// Gets all the endpoint categories that the REST API supports.
    ///
    /// See also: [`api_info`]
    async fn get_api_info(&self) -> std::result::Result<GetResponse, GithubRestError> {
        api_info(self.http_client()).await
    }

    /// Gets a random line from the zen of GitHub.
    ///
    /// See also: [`GetZen`]
    ///
    /// [`GetZen`]: github_api::end_points::EndPoints::GetZen
    async fn zen(&self) -> std::result::Result<String, GithubRestError> {
        zen(self.http_client()).await
    }
}

// TODO: Method impls
/// Where the magic happens.
pub struct Client<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    handler: T,
    max_payload_size: ByteUnit,
    http_client: HttpClient,
}

#[async_trait]
impl<T> GitHubClient for Client<T>
where
    T: std::fmt::Debug + EventHandler + Send + Sync,
{
    type HttpClient = HttpClient;
    type EventHandler = T;

    // TODO: User-facing API to set this
    async fn run(&self) -> Result<()> {
        Ok(())
    }

    fn http_client(&self) -> &Self::HttpClient {
        &self.http_client
    }

    fn event_handler(&self) -> &T {
        &self.handler
    }

    fn payload_size(&self) -> ByteUnit {
        self.max_payload_size.mebibytes()
    }
}

impl<T> Client<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    /// Creates a new [`Client`].
    pub fn new(handler: T, auth: Option<Authorization>, user_agent: Option<String>, payload_size: Option<u64>) -> Self {
        Self {
            handler,
            max_payload_size: payload_size.unwrap_or(8).mebibytes(),
            http_client: HttpClient::new(auth, user_agent),
        }
    }

    /// Updates the authorization parameter in the current [`Client`] instance.
    pub fn set_auth(mut self, auth: Authorization) -> Self {
        self.http_client.set_auth(auth);
        self
    }
}

impl Default for Client<DefaultEventHandler> {
    fn default() -> Client<DefaultEventHandler> {
        Client {
            handler: DefaultEventHandler,
            max_payload_size: 8.mebibytes(),
            http_client: HttpClient::new(None, None),
        }
    }
}
