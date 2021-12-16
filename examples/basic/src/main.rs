use anyhow::Result;

use octocat_rs::github::{GitHub, GitHubApplication};

#[tokio::main]
async fn main() -> Result<()> {
    let github_client = GitHub::new("", "");
    github_client.start().await?;

    Ok(())
}
