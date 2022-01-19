use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    methods::util,
    model::{
        commits::{comments, comments::CommitComment},
        user::User,
    },
    GithubRestError, Requester,
};

pub type Commits = Vec<Commit>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub node_id: String,
    pub commit: comments::CommitComment,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: User,
    pub committer: User,
    pub parents: Vec<nested::Parent>,
}

impl Commit {
    pub async fn add_comment_arc(
        &self,
        client: Arc<&impl Requester>,
        body: String,
        path: Option<String>,
        position: Option<String>,
    ) -> Result<CommitComment, GithubRestError> {
        self.add_comment(*client, body, path, position).await
    }
    /// Adds a comment to the current instance.
    pub async fn add_comment(
        &self,
        client: &impl Requester,
        body: String,
        path: Option<String>,
        position: Option<String>,
    ) -> Result<CommitComment, GithubRestError> {
        util::helper_for_helper_for_helper(client, self.html_url.clone(), self.sha.clone(), body, path, position).await
    }
}

pub mod nested {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Tree {
        pub sha: String,
        pub url: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Verification {
        pub verified: bool,
        pub reason: String,
        pub signature: Option<String>,
        pub payload: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Parent {
        pub sha: String,
        pub url: String,
        pub html_url: String,
    }
}
