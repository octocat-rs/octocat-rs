use anyhow::Result;
use async_trait::async_trait;
use github_rest::{
    methods::{api_info, get_commits, get_issues, get_pulls, prelude::GetResponse, zen},
    structs::{Commit, Issues, Pulls},
    GithubRestError, Requester,
};
use serde_json::Value;

use crate::github::{handler::EventHandler, util::Authorization, DefaultEventHandler, HttpClient};

// TODO: Fix the issues on github-rest so that this alias is unnecessary
pub type Commits = Vec<Commit>;

#[async_trait]
pub trait GitHubClient {
    type HttpClient: Requester + Send + Sync;
    type EventHandler: EventHandler + Send + Sync;

    /// Code that the implementer wishes to be run *before* the event listener
    /// is started.
    async fn run(&self) -> Result<()>;

    async fn start(&self) -> Result<()> {
        // TODO: Runtime
        self.run().await.expect("Starting application: User-defined code");

        self.listener().await
    }

    // TODO: Rocket config (logs, max payload size, ...)
    // TODO: Webhook types
    async fn listener(&self) -> Result<()> {
        let figment = rocket::Config::figment().merge(("port", self.event_handler().webhook_port()));

        #[post("/", format = "application/json", data = "<testing>")]
        async fn tester(testing: String) {
            dbg!(serde_json::from_str::<'_, Value>(testing.as_str()).unwrap());
        }

        rocket::custom(figment)
            .mount(self.event_handler().route(), routes![tester])
            .launch()
            .await?;

        Ok(())
    }

    fn http_client(&self) -> &Self::HttpClient;

    fn event_handler(&self) -> &Self::EventHandler;

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
#[allow(dead_code)]
pub struct Client<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    handler: T,
    http_client: HttpClient,
}

#[async_trait]
impl<T> GitHubClient for Client<T>
where
    T: std::fmt::Debug + EventHandler + Send + Sync,
{
    type HttpClient = HttpClient;
    type EventHandler = T;

    async fn run(&self) -> Result<()> {
        Ok(())
    }

    fn http_client(&self) -> &Self::HttpClient {
        &self.http_client
    }

    fn event_handler(&self) -> &T {
        &self.handler
    }
}

impl<T> Client<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    /// Creates a new [`Client`].
    pub fn new(handler: T, auth: Option<Authorization>, user_agent: Option<String>) -> Self {
        Self {
            handler,
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
            http_client: HttpClient::new(None, None),
        }
    }
}
