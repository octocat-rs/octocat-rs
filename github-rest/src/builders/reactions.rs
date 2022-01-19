use crate::{
    methods::react_to_commit_comment,
    model::reactions::{CommitCommentReactionCreated, Reaction},
    GithubRestError, Requester,
};

/// * tags reactions
/// * post `/repos/{owner}/{repo}/comments/{comment_id}/reactions`
/// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-commit-comment>
///
/// Create reaction for a commit comment
/// Create a reaction to a [commit comment](https://docs.github.com/rest/reference/repos#comments). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.
pub struct ReactionOnCommitCommentBuilder {
    owner_and_repo: (String, String),
    comment_id: i64,
    reaction: Reaction,
}

impl ReactionOnCommitCommentBuilder {
    pub fn new(owner: String, repo: String, comment_id: i64, reaction: Reaction) -> Self {
        ReactionOnCommitCommentBuilder {
            owner_and_repo: (owner, repo),
            comment_id,
            reaction,
        }
    }

    pub async fn execute<T>(self, client: &T) -> Result<CommitCommentReactionCreated, GithubRestError>
    where
        T: Requester,
    {
        react_to_commit_comment(
            client,
            self.owner_and_repo.0,
            self.owner_and_repo.1,
            self.comment_id,
            self.reaction,
        )
        .await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::{
        builders::ReactionOnCommitCommentBuilder,
        client::DefaultRequest,
        model::{reactions::Reaction, Reaction},
    };

    #[cfg(feature = "builders")]
    #[tokio::test]
    async fn test_react_to_comment_on_commit() {
        let res = ReactionOnCommitCommentBuilder::new(
            "octocat-rs".to_owned(),
            "octocat-rs".to_owned(),
            62802084,
            Reaction::Rocket,
        )
        .execute(&DefaultRequest::new_none())
        .await;

        dbg!(res.unwrap());
    }
}
