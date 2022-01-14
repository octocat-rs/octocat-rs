use crate::{
    methods::{comment_on_commit, prelude::CommitComment, CommentOnCommitBody},
    GithubRestError, Requester,
};

/// * tags repos
/// * post `/repos/{owner}/{repo}/commits/{commit_sha}/comments`
/// * docs <https://docs.github.com/rest/reference/repos#create-a-commit-comment>
///
/// Create a commit comment
/// Create a comment for a commit using its `:commit_sha`.
///
/// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
pub struct CommentOnCommitBuilder {
    data: (String, String, String),
    body: CommentOnCommitBody,
}

impl CommentOnCommitBuilder {
    pub fn new(user: String, repo: String, hash: String, content: String) -> Self {
        CommentOnCommitBuilder {
            data: (user, repo, hash),
            body: CommentOnCommitBody {
                body: content,
                path: None,
                position: None,
                line: None,
            },
        }
    }
    pub fn body(mut self, body: String) -> CommentOnCommitBuilder {
        self.body.body = body;
        self
    }
    pub fn path(mut self, path: String) -> CommentOnCommitBuilder {
        self.body.path = Some(path);
        self
    }
    pub fn position(mut self, position: String) -> CommentOnCommitBuilder {
        self.body.position = Some(position);
        self
    }

    pub async fn execute<T>(self, client: &T) -> Result<CommitComment, GithubRestError>
    where
        T: Requester,
    {
        comment_on_commit(client, self.data.0, self.data.1, self.data.2, self.body).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::{builders::CommentOnCommitBuilder, client::DefaultRequest};

    #[tokio::test]
    async fn test_comment_on_commit() {
        let comment = CommentOnCommitBuilder::new(
            "octocat-rs".to_owned(),
            "github-rest".to_owned(),
            "2eb7eeba66a6adf2168391d0cd6dcac995a34489".to_owned(),
            "Losing my mind".to_owned(),
        );

        // You'll need to add your auth to get this to pass
        let a = comment.execute(&DefaultRequest::new_none()).await.unwrap();

        dbg!(a);
    }
}
