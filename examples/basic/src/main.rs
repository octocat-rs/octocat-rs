use octocat_rs::github::GitHub;

#[tokio::main]
async fn main() {
    let github_client = GitHub::new("USERNAME", "OAUTH_TOKEN");
    github_client.run().await;
}
