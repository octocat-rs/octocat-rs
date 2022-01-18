use std::sync::Arc;

use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

use crate::{
    methods::{get_commit, util, GetCommitBody},
    model::{nested::CommitComment, Organization, Repository},
    GithubRestError, Requester,
};

use super::User;

pub type Commits = Vec<Commit>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub node_id: String,
    pub commit: nested::CommitComment,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitCommentEvent {
    pub action: CommitCommentAction,
    pub comment: CommitComment,
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum CommitCommentAction {
    Created,
}

impl Default for CommitCommentAction {
    fn default() -> Self {
        Self::Created
    }
}

pub mod nested {
    use serde::{Deserialize, Serialize};

    use crate::{
        methods::{react_to_commit_comment, util},
        GithubRestError, Requester,
    };
    // TODO: Create better names for these model
    use crate::model::{CommitCommentReactionCreated, Reaction, Reactions, User};

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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Association {
        Collaborator,
        Contributor,
        FirstTimer,
        FirstTimeContributor,
        Mannequin,
        Member,
        None,
        Owner,
    }

    impl Default for Association {
        fn default() -> Self {
            Association::None
        }
    }
}
