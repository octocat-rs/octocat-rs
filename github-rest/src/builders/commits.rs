use crate::{
    builders::Builder,
    methods::{comment_on_commit, CommitCommentBody},
    model::commits::comments::CommitComment,
    GithubRestError, Requester,
};
use async_trait::async_trait;

/// * tags repos
/// * post `/repos/{owner}/{repo}/commits/{commit_sha}/comments`
/// * docs <https://docs.github.com/rest/reference/repos#create-a-commit-comment>
///
/// Create a commit comment
/// Create a comment for a commit using its `:commit_sha`.
///
/// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
#[derive(Default, Clone)]
pub struct CommitCommentBuilder {
    owner: String,
    repo: String,
    sha: String,
    options: CommitCommentBody,
}

impl CommitCommentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn owner<T: Into<String>>(mut self, owner: T) -> CommitCommentBuilder {
        self.owner = owner.into();
        self
    }

    pub fn repo<T: Into<String>>(mut self, repo: T) -> CommitCommentBuilder {
        self.repo = repo.into();
        self
    }

    pub fn sha<T: Into<String>>(mut self, sha: T) -> CommitCommentBuilder {
        self.sha = sha.into();
        self
    }

    pub fn body<T: Into<String>>(mut self, body: T) -> CommitCommentBuilder {
        self.options.body = body.into();
        self
    }

    pub fn path<T: Into<String>>(mut self, path: T) -> CommitCommentBuilder {
        self.options.path = Some(path.into());
        self
    }

    pub fn position<T: Into<String>>(mut self, position: T) -> CommitCommentBuilder {
        self.options.position = Some(position.into());
        self
    }
}

#[async_trait]
impl Builder for CommitCommentBuilder {
    type Response = CommitComment;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        comment_on_commit(client, self.owner, self.repo, self.sha, self.options).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::{
        builders::{Builder, CommitCommentBuilder},
        client::DefaultRequest,
    };

    #[tokio::test]
    async fn test_comment_on_commit() {
        let comment = CommitCommentBuilder::new()
            .owner("octocat-rs")
            .repo("github-rest")
            .sha("2eb7eeba66a6adf2168391d0cd6dcac995a34489")
            .body("Losing my mind");

        // You'll need to add your auth to get this to pass
        let a = comment.execute(&DefaultRequest::new_none()).await.unwrap();

        dbg!(a);
    }
}
