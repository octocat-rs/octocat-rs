use octocat_rs::{
    rest::{
        builders::{Builder, CommitCommentBuilder, GetIssuesBuilder},
        GithubRestError,
    },
    Authorization, HttpClient,
};

const TOKEN: String = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env var must be set!");
const ACCOUNT: String = std::env::var("GITHUB_ACCOUNT").expect("GITHUB_ACCOUNT env var must be set!");

#[tokio::main]
async fn main() -> Result<(), GithubRestError> {
    let mut http_client = HttpClient::new_none();

    // TODO: Replace with GetCommitsBuilder once that's created
    GetIssuesBuilder::new()
        .owner("octocat-rs")
        .repo("octocat-rs")
        .per_page(50.to_string())
        .execute(&http_client)
        .await?;

    http_client.set_auth(Authorization::PersonalToken {
        username: ACCOUNT,
        token: TOKEN,
    });

    let res = CommitCommentBuilder::new()
        .owner("octocat-rs")
        .repo("octocat-rs")
        .sha("fcc8348f8286d05976090a9086d64eefb90e3f8b")
        .body("Some text here")
        .execute(&http_client)
        .await?;

    // Prints the URL at which you can find the comment you've just made.
    dbg!(res.html_url);

    Ok(())
}
