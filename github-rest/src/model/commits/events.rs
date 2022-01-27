use crate::{
    methods::{get_commit, GetCommitBody},
    model::{
        commits::{comments::CommitComment, Commit},
        event_types::macros::repo_origin,
        organizations::Organization,
        prelude::*,
        repositories::Repository,
        user::User,
    },
    GithubRestError, Requester,
};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#commit_comment>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitCommentEvent {
    pub action: CommitCommentAction,
    pub comment: CommitComment,
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum CommitCommentAction {
    Created,
}

impl CommitCommentEvent {
    /// Get the commit that the current comment refers to.
    ///
    /// See also: <https://docs.github.com/en/rest/reference/commits#get-a-commit>
    pub async fn get_commit<T>(&self, client: &T, options: Option<&GetCommitBody>) -> Result<Commit, GithubRestError>
    where
        T: Requester,
    {
        get_commit(
            client,
            self.repository.owner.login.clone(),
            self.repository.name.clone(),
            self.comment.commit_id.clone(),
            options,
        )
        .await
    }
}

impl Default for CommitCommentAction {
    fn default() -> Self {
        Self::Created
    }
}

repo_origin!(CommitCommentEvent);
