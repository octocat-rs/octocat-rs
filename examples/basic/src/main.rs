use anyhow::Result;

use octocat_rs::github::{GitHubApplication, PersonalClient};

#[tokio::main]
async fn main() -> Result<()> {
    let github_client = PersonalClient::new("", "");
    github_client.start().await?;

    Ok(())
}
