use crate::{
    methods::{react_to_commit_comment, util},
    model::{
        commits::association::Association,
        prelude::*,
        reactions::{CommitCommentReactionCreated, Reaction, ReactionRollup},
        user::SimpleUser,
    },
    GithubRestError, Requester,
};

/// <https://docs.github.com/en/rest/commits/comments#get-a-commit-comment=>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitComment {
    pub url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub user: Option<SimpleUser>,
    pub position: Option<i64>,
    pub line: Option<i64>,
    pub path: Option<String>,
    pub commit_id: String,
    pub body: String,
    pub author_association: Association,
    pub created_at: String,
    pub updated_at: String,
    pub reactions: Option<ReactionRollup>,
}

impl CommitComment {
    pub async fn add_reaction<T>(
        &self,
        client: &T,
        reaction: Reaction,
    ) -> Result<CommitCommentReactionCreated, GithubRestError>
    where
        T: Requester,
    {
        let (owner, repo) = util::owner_and_repo(self.html_url.clone());

        react_to_commit_comment(client, owner, repo, self.id, reaction).await
    }
}
