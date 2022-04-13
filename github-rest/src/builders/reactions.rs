use crate::{
    builders::{builder, builder_setters, builder_string_setters, Builder},
    methods::react_to_commit_comment,
    model::reactions::{CommitCommentReactionCreated, Reaction},
    GithubRestError, Requester,
};
use async_trait::async_trait;

builder!(
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/comments/{comment_id}/reactions`
    /// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-commit-comment>
    ///
    /// Create reaction for a commit comment
    /// Create a reaction to a [commit comment](https://docs.github.com/rest/reference/repos#comments). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.
    CommentReactionBuilder {
        owner: String,
        repo: String,
        comment_id: i64,
        reaction: Reaction
    }
);

builder_string_setters!(CommentReactionBuilder { owner, repo });
builder_setters!(CommentReactionBuilder {
    comment_id: i64,
    reaction: Reaction
});

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
        methods::util,
        model::reactions::Reaction,
    };

    #[tokio::test]
    async fn test_react_to_commit_comment() {
        let res = CommentReactionBuilder::new()
            .owner("octocat-rs")
            .repo("octocat-rs")
            .comment_id(62802084)
            .reaction(Reaction::Rocket)
            .execute(&util::github_auth())
            .await;

        dbg!(res.unwrap());
    }
}
