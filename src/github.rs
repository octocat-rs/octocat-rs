use reqwest::{
    header::{HeaderMap, USER_AGENT},
    RequestBuilder,
};

use anyhow::Result;
use async_trait::async_trait;

/// Used to represent the current [authorization method](https://docs.github.com/en/rest/overview/resources-in-the-rest-api#authentication).
#[derive(Debug, Clone)]
pub enum Authorization {
    Personal {
        username: String,
        access_token: String,
    },
    OAuth(String),
    Sso(String),
}

/// A simple implementation of [`GitHubApplication`], intended for use in small applications.
pub struct GitHub {
    client: reqwest::Client,
    auth: Authorization,
}

#[async_trait]
impl GitHubApplication for GitHub {
    fn new(username: &str, access_token: &str) -> Self {
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

    fn new_with_sso(_token: &str) -> Self {
        todo!()
    }

    fn current_auth_method(&self) -> Authorization {
        todo!()
    }

    fn http_request(&self, builder: RequestBuilder) -> RequestBuilder {
        match self.auth.clone() {
            Authorization::Personal {
                username,
                access_token,
            } => builder.basic_auth(username, Some(access_token)),
            _ => todo!(),
        }
    }

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

// TODO: Work out how requests will be handled; will probably be done with some sort of util module

/// A trait to be implemented by you, the user.
#[async_trait]
pub trait GitHubApplication {
    /// Basic username + OAuth token authentication.
    fn new(username: &str, token: &str) -> Self;

    /// For accessing organizations that enforce SAML SSO with a personal access token.
    ///
    /// Further reading: <https://docs.github.com/en/rest/overview/other-authentication-methods#authenticating-for-saml-sso>
    fn new_with_sso(token: &str) -> Self;

    /// Helper function for getting the current authorization method.
    fn current_auth_method(&self) -> Authorization;

    /// Wrapper function for getting an HTTP client; GitHub API requires the user agent header to be set.
    fn http_request(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder;

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
