use octocat_rs::github::GitHub;

#[tokio::main]
async fn main() {
    let github_client = GitHub::new_with_sso("TOKEN");
    github_client.run().await;
}
