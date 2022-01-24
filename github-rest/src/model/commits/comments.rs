use super::super::prelude::*;

use crate::{
    methods::{react_to_commit_comment, util},
    model::{
        commits::association::Association,
        reactions::{CommitCommentReactionCreated, Reaction, Reactions},
        user::User,
    },
    GithubRestError, Requester,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitComment {
    pub html_url: String,
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub body: String,
    pub path: Option<String>,
    pub position: Option<i64>,
    pub line: Option<i64>,
    pub commit_id: String,
    pub author_association: Association,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
    pub reactions: Reactions,
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
