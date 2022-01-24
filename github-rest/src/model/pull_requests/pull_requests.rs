use super::super::prelude::*;

use crate::model::{
    pull_requests::nested::{HeadBase, Links},
    user::User,
};

pub type Pulls = Vec<PullRequest>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub number: i64,
    pub state: PullRequestState,
    pub locked: bool,
    pub title: String,
    pub user: User,
    pub body: Value,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Value,
    pub merged_at: Value,
    pub merge_commit_sha: Value,
    pub assignee: Value,
    pub assignees: Vec<Value>,
    pub requested_reviewers: Vec<Value>,
    pub requested_teams: Vec<Value>,
    pub labels: Vec<Value>,
    pub milestone: Value,
    pub draft: bool,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub head: HeadBase,
    pub base: HeadBase,
    #[serde(rename = "_links")]
    pub links: Links,
    pub author_association: String,
    pub auto_merge: Value,
    pub active_lock_reason: Value,
    pub merged: bool,
    pub mergeable: Option<bool>,
    // Don't know if these two can be null so just to be safe(?)
    pub rebaseable: Option<bool>,
    pub mergeable_state: String,
    // Docs really didn't help me when I attempted to create an enum for this
    pub merged_by: Value,
    pub comments: i64,
    pub review_comments: i64,
    pub maintainer_can_modify: bool,
    pub commits: i64,
    pub additions: i64,
    pub deletions: i64,
    pub changed_files: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestState {
    Closed,
    Merged,
    Open,
}

impl Default for PullRequestState {
    fn default() -> Self {
        PullRequestState::Open
    }
}

pub mod nested {
    use serde::{Deserialize, Serialize};

    use crate::model::{repositories::Repository, user::User};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct HeadBase {
        pub label: String,
        #[serde(rename = "ref")]
        pub ref_field: String,
        pub sha: String,
        pub user: User,
        pub repo: Repository,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Links {
        #[serde(rename = "self")]
        pub self_field: HRef,
        pub html: HRef,
        pub issue: HRef,
        pub comments: HRef,
        pub review_comments: HRef,
        pub review_comment: HRef,
        pub commits: HRef,
        pub statuses: HRef,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct HRef {
        pub href: String,
    }
}
