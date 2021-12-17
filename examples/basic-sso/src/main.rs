use octocat_rs::github::{GitHubApplication, GitHubSsoClient};

#[tokio::main]
async fn main() {
    let github_client = GitHubSsoClient::new("TOKEN");
    github_client.run().await;
}
