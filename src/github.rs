use reqwest::{
    header::{HeaderMap, AUTHORIZATION, USER_AGENT},
    RequestBuilder,
};

use anyhow::Result;
use async_trait::async_trait;

/// A simple, pre-made implementation of [`GitHubApplication`] for personal
/// access tokens.
pub struct GitHubPersonalClient {
    client: reqwest::Client,
    auth: Authorization,
}

/// Used to hold the credentials for [`GitHubPersonalClient`].
#[derive(Debug, Clone)]
pub enum Authorization {
    Personal {
        username: String,
        access_token: String,
    },
}
#[async_trait]
impl GitHubApplication for GitHubPersonalClient {
    async fn run(&self) -> Result<()> {
        let res = self
            .http_request(self.client.get("https://api.github.com/user"))
            .send()
            .await?
            .text()
            .await?;

        dbg!(res);
        todo!()
    }
}

impl GitHubPersonalClient {
    /// Basic username + personal access token authentication.
    pub fn new(username: &str, access_token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "Mr. Wu Han".parse().unwrap()); // I'm a comedian

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            client,
            auth: Authorization::Personal {
                username: username.to_owned(),
                access_token: access_token.to_owned(),
            },
        }
    }

    /// Wrapper function for getting an HTTP client; GitHub API requires the
    /// user agent header to be set.
    pub fn http_request(&self, builder: RequestBuilder) -> RequestBuilder {
        match self.auth.clone() {
            Authorization::Personal {
                username,
                access_token,
            } => builder.basic_auth(username, Some(access_token)),
        }
    }
}

/// An implementation of [`GitHubApplication`] for accessing organizations that
/// enforce SAML SSO with a personal access token.
///
/// Further reading: <https://docs.github.com/en/rest/overview/other-authentication-methods#authenticating-for-saml-sso>
pub struct GitHubSsoClient {
    client: reqwest::Client,
}

#[async_trait]
impl GitHubApplication for GitHubSsoClient {
    async fn run(&self) -> Result<()> {
        todo!()
    }
}

impl GitHubSsoClient {
    pub fn new(token: &str) -> Self {
        let token = format!("token {}", token);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, token.parse().unwrap());

        Self {
            client: reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }
}

// TODO: Shared request methods
/// A trait to be implemented by you, the user.
#[async_trait]
pub trait GitHubApplication {
    /// The code that is run when your application starts. Called by [`start`].
    ///
    /// [`start`]: GitHubApplication::start
    async fn run(&self) -> Result<()>;

    // TODO: Settings interface
    async fn start(&self) -> Result<()> {
        // TODO: Proper logging etc
        self.run().await
    }
}
