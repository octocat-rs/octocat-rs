use anyhow::Result;
use async_trait::async_trait;
use github_rest::{
    methods::{api_info, get_commits, get_issues, get_pulls, prelude::GetResponse, zen},
    structs::{Commits, Issues, Pulls},
    GithubRestError, Requester,
};

use crate::github::{handler::EventHandler, util::Authorization, DefaultEventHandler, HttpClient};

#[async_trait]
pub trait GitHubClient {
    type HttpClient: Requester + Send + Sync;

    /// The code that the implementer wants to be run at startup.
    async fn run(&self) -> Result<()>;

    async fn start(&self) -> Result<()> {
        // TODO: Runtime
        self.run().await
    }

    fn http_client(&self) -> &Self::HttpClient;

    /// Gets all commits from a repository.
    ///
    /// See also: [`get_commits`]
    async fn get_all_commits(&self, owner: &str, repo: &str) -> std::result::Result<Commits, GithubRestError> {
        get_commits(self.http_client(), owner.to_owned(), repo.to_owned(), None).await
    }

    /// Gets all issues from a repository.
    ///
    /// See also: [`get_issues`]
    async fn get_all_issues(&self, owner: &str, repo: &str) -> std::result::Result<Issues, GithubRestError> {
        get_issues(self.http_client(), owner.to_owned(), repo.to_owned(), None).await
    }

    /// Gets all pull requests from a repository.
    ///
    /// See also: [`get_pulls`]
    async fn get_all_pulls(&self, owner: &str, repo: &str) -> std::result::Result<Pulls, GithubRestError> {
        get_pulls(self.http_client(), owner.to_owned(), repo.to_owned(), None).await
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

// TODO: HTTP client, Client trait, method impls
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

    async fn run(&self) -> Result<()> {
        Ok(())
    }

    fn http_client(&self) -> &Self::HttpClient {
        &self.http_client
    }
}

impl<T> Client<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    /// Creates a new [`Client`].
    pub fn new(handler: T, auth: Option<Authorization>) -> Self {
        Self {
            handler,
            http_client: HttpClient::new(auth),
        }
    }

    /// Updates the authorization parameter in the current [`Client`] instance.
    pub fn set_auth(self, auth: Authorization) -> Self {
        Self {
            handler: self.handler,
            http_client: HttpClient::new(Some(auth)),
        }
    }
}

impl Default for Client<DefaultEventHandler> {
    fn default() -> Client<DefaultEventHandler> {
        Client {
            handler: DefaultEventHandler,
            http_client: HttpClient::new(None),
        }
    }
}
