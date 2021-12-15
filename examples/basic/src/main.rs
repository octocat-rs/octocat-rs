use octocat_rs::github::GitHub;

#[tokio::main]
async fn main() {
    let github_client = GitHub::new("API_KEY");
    github_client.run().await;
}
