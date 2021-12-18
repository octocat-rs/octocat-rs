use reqwest::{
    header::{HeaderMap, USER_AGENT},
    RequestBuilder,
};

use crate::github::{traits::GitHubApplication, Authorization};
use anyhow::Result;
use async_trait::async_trait;

/// A simple, pre-made implementation of [`GitHubApplication`] for personal
/// access tokens.
pub struct PersonalClient {
    client: reqwest::Client,
    auth: Authorization,
}

#[async_trait]
impl GitHubApplication for PersonalClient {
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

impl PersonalClient {
    /// Basic username + personal access token authentication.
    pub fn new(username: &str, access_token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "Mr. Wu Han".parse().unwrap()); // I'm a comedian

        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();

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
            Authorization::Personal { username, access_token } => builder.basic_auth(username, Some(access_token)),
        }
    }
}
