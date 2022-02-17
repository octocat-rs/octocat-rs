use crate::{
    builders::Builder,
    methods::{comment_on_commit, react_to_commit_comment, CommentOnCommitBody},
    model::{
        commits::comments::CommitComment,
        reactions::{CommitCommentReactionCreated, Reaction},
    },
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
#[derive(Default)]
pub struct CommentOnCommitBuilder {
    owner: String,
    repo: String,
    sha: String,
    options: CommentOnCommitBody,
}

impl CommentOnCommitBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn owner<T: Into<String>>(mut self, owner: T) -> CommentOnCommitBuilder {
        self.owner = owner.into();
        self
    }

    pub fn repo<T: Into<String>>(mut self, repo: T) -> CommentOnCommitBuilder {
        self.repo = repo.into();
        self
    }

    pub fn sha<T: Into<String>>(mut self, sha: T) -> CommentOnCommitBuilder {
        self.sha = sha.into();
        self
    }

    pub fn body<T: Into<String>>(mut self, body: T) -> CommentOnCommitBuilder {
        self.options.body = body.into();
        self
    }

    pub fn path<T: Into<String>>(mut self, path: T) -> CommentOnCommitBuilder {
        self.options.path = Some(path.into());
        self
    }

    pub fn position<T: Into<String>>(mut self, position: T) -> CommentOnCommitBuilder {
        self.options.position = Some(position.into());
        self
    }
}

#[async_trait]
impl Builder for CommentOnCommitBuilder {
    type Response = CommitComment;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        comment_on_commit(client, self.owner, self.repo, self.sha, self.options).await
    }
}

#[derive(Default)]
pub struct ReactToCommitCommentBuilder {
    owner: String,
    repo: String,
    comment_id: i64,
    reaction: Reaction,
}

impl ReactToCommitCommentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn owner<T: Into<String>>(mut self, owner: T) -> ReactToCommitCommentBuilder {
        self.owner = owner.into();
        self
    }
    pub fn repo<T: Into<String>>(mut self, repo: T) -> ReactToCommitCommentBuilder {
        self.repo = repo.into();
        self
    }
    pub fn comment_id(mut self, comment_id: i64) -> ReactToCommitCommentBuilder {
        self.comment_id = comment_id;
        self
    }
    pub fn reaction(mut self, reaction: Reaction) -> ReactToCommitCommentBuilder {
        self.reaction = reaction;
        self
    }
}

#[async_trait]
impl Builder for ReactToCommitCommentBuilder {
    type Response = CommitCommentReactionCreated;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        react_to_commit_comment(client, self.owner, self.repo, self.comment_id, self.reaction).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::{
        builders::{Builder, CommentOnCommitBuilder, ReactToCommitCommentBuilder},
        client::DefaultRequest,
    };

    #[tokio::test]
    async fn test_comment_on_commit() {
        let comment = CommentOnCommitBuilder::new()
            .owner("octocat-rs")
            .repo("github-rest")
            .sha("2eb7eeba66a6adf2168391d0cd6dcac995a34489")
            .body("Losing my mind");

        // You'll need to add your auth to get this to pass
        let a = comment.execute(&DefaultRequest::new_none()).await.unwrap();

        dbg!(a);
    }

    #[tokio::test]
    async fn test_react_to_commit_comment() {
        let mut comment_reaction = ReactToCommitCommentBuilder::default();
        comment_reaction = comment_reaction.owner("Owner".to_owned());

        // You'll need to add your auth to get this to pass
        let a = comment_reaction.execute(&DefaultRequest::new_none()).await.unwrap();

        dbg!(a);
    }
}
