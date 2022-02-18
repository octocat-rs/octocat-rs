use crate::{
    builders::Builder,
    methods::react_to_commit_comment,
    model::reactions::{CommitCommentReactionCreated, Reaction},
    GithubRestError, Requester,
};
use async_trait::async_trait;

/// * tags reactions
/// * post `/repos/{owner}/{repo}/comments/{comment_id}/reactions`
/// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-commit-comment>
///
/// Create reaction for a commit comment
/// Create a reaction to a [commit comment](https://docs.github.com/rest/reference/repos#comments). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.
#[derive(Default, Clone)]
pub struct CommentReactionBuilder {
    owner: String,
    repo: String,
    comment_id: i64,
    reaction: Reaction,
}

impl CommentReactionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn owner<T: Into<String>>(mut self, owner: T) -> CommentReactionBuilder {
        self.owner = owner.into();
        self
    }
    pub fn repo<T: Into<String>>(mut self, repo: T) -> CommentReactionBuilder {
        self.repo = repo.into();
        self
    }
    pub fn comment_id(mut self, comment_id: i64) -> CommentReactionBuilder {
        self.comment_id = comment_id;
        self
    }
    pub fn reaction(mut self, reaction: Reaction) -> CommentReactionBuilder {
        self.reaction = reaction;
        self
    }
}

#[async_trait]
impl Builder for CommentReactionBuilder {
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
        builders::{Builder, CommentReactionBuilder},
        client::DefaultRequest,
        model::reactions::Reaction,
    };

    #[tokio::test]
    async fn test_react_to_commit_comment() {
        let res = CommentReactionBuilder::new()
            .owner("octocat-rs")
            .repo("octocat-rs")
            .comment_id(62802084)
            .reaction(Reaction::Rocket)
            .execute(&DefaultRequest::new_none())
            .await;

        // You'll need to add your auth to get this to pass
        dbg!(res.unwrap());
    }
}
