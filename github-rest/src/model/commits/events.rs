use crate::{
    methods::{get_commit, GetCommitBody},
    model::{
        commits::{comments::CommitComment, Commit},
        event_types::{macros::repo_origin, RepoEventInfo},
        prelude::*,
    },
    GithubRestError, Requester,
};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#commit_comment>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitCommentEvent {
    pub action: CommitCommentAction,
    pub comment: CommitComment,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum CommitCommentAction {
    #[default]
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
            self.event_info.repository.owner.login.clone(),
            self.event_info.repository.name.clone(),
            self.comment.commit_id.clone(),
            options,
        )
        .await
    }
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#status>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusEvent {
    pub id: usize,
    pub sha: String,
    pub description: Option<String>,
    pub target_url: Option<String>,
    pub commit: Commit,
    pub state: StatusState,
    pub branches: Vec<nested::Branch>,
    pub created_at: Value,
    pub updated_at: Value,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum StatusState {
    Pending,
    Success,
    Failure,
    Error,
}

pub mod nested {
    use crate::model::prelude::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Branch {
        pub name: String,
        pub commit: NestedCommit,
        pub protected: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct NestedCommit {
        pub sha: String,
        pub url: String,
    }
}

repo_origin!(CommitCommentEvent);
repo_origin!(StatusEvent);
