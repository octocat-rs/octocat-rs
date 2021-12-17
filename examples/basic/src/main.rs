use anyhow::Result;

use octocat_rs::github::{GitHubApplication, GitHubPersonalClient};

#[tokio::main]
async fn main() -> Result<()> {
    let github_client = GitHubPersonalClient::new("", "");
    github_client.start().await?;

    Ok(())
}
