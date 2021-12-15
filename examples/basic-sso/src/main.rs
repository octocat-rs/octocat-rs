use octocat_rs::github::GitHubApplication;

#[tokio::main]
async fn main() {
    let github_client = GitHubApplication::new_with_sso("TOKEN");
    github_client.run().await;
}
