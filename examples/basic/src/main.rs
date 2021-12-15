use anyhow::Result;
use async_trait::async_trait;
use octocat_rs::github::{AuthMethod, GitHubApplication};

#[tokio::main]
async fn main() -> Result<()> {
    let github_client = GitHub::new("USERNAME", "OAUTH_TOKEN");
    github_client.run().await?;

    Ok(())
}

/// An API client for github.
#[allow(dead_code)]
pub struct GitHub {
    api_key: String,
    username: Option<String>,
    auth_method: AuthMethod,
    http_client: reqwest::Client,
}

#[async_trait]
impl GitHubApplication for GitHub {
    fn new(username: &str, token: &str) -> Self {
        Self {
            api_key: token.to_owned(),
            username: Some(username.to_owned()),
            auth_method: AuthMethod::OAuthToken,
            ..Default::default()
        }
    }

    fn new_with_sso(token: &str) -> Self {
        Self {
            api_key: token.to_owned(),
            username: None,
            auth_method: AuthMethod::Sso,
            ..Default::default()
        }
    }

    fn current_auth_method(&self) -> AuthMethod {
        self.auth_method
    }

    async fn run(&self) -> Result<()> {
        todo!()
    }
}
