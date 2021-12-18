use anyhow::Result;
use octocat_rs::{github, github::traits::GitHubApplication};

#[tokio::main]
async fn main() -> Result<()> {
    let github_client = github::PersonalClient::new("", "");
    github_client.start().await?;

    Ok(())
}
