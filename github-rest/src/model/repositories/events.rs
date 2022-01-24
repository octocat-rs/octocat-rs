use std::sync::Arc;

use super::super::prelude::*;

use crate::{
    methods::util,
    model::{
        commits::comments::CommitComment,
        repositories::{
            events::nested::{Commit, HeadCommit, Pusher},
            Repository,
        },
        user::User,
    },
    GithubRestError, Requester,
};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#push>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub before: String,
    pub after: String,
    pub repository: Repository,
    pub pusher: Pusher,
    pub sender: User,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Value,
    pub compare: String,
    pub commits: Vec<Commit>,
    pub head_commit: Option<HeadCommit>,
}

impl PushEvent {
    /// Adds a comment to the commit that triggered the event.
    ///
    /// See also: <https://docs.github.com/en/rest/reference/commits#create-a-commit-comment>
    pub async fn add_comment_to_commit(
        &self,
        client: Arc<&impl Requester>,
        body: String,
        path: Option<String>,
        position: Option<String>,
    ) -> Result<CommitComment, GithubRestError> {
        let hc = self.head_commit.as_ref().unwrap();

        util::helper_for_helper_for_helper(
            *client,
            hc.url.clone(),
            hc.id.clone(),
            body,
            path,
            position,
        )
        .await
    }
}

pub mod nested {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use crate::model::user::SimpleUser;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Pusher {
        pub name: String,
        pub email: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Commit {
        pub id: String,
        pub tree_id: String,
        pub distinct: bool,
        pub message: String,
        pub timestamp: String,
        pub url: String,
        pub author: SimpleUser,
        pub committer: SimpleUser,
        pub added: Vec<String>,
        pub removed: Vec<Value>,
        pub modified: Vec<Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct HeadCommit {
        pub id: String,
        pub tree_id: String,
        pub distinct: bool,
        pub message: String,
        pub timestamp: String,
        pub url: String,
        pub author: SimpleUser,
        pub committer: SimpleUser,
        pub added: Vec<String>,
        pub removed: Vec<Value>,
        pub modified: Vec<Value>,
    }
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#star>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarEvent {
    pub action: StarAction,
    pub starred_at: String,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum StarAction {
    Created,
    Deleted,
}

// TODO: Watch event
/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#fork>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForkEvent {
    forkee: Repository,
    repository: Repository,
    sender: User,
}
