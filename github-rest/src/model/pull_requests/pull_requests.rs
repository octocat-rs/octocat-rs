use crate::model::{
    commits::association::Association,
    issues::{milestones::Milestone, Label},
    prelude::*,
    pull_requests::nested::{AutoMerge, HeadBase, Links},
    user::SimpleUser,
};

pub type Pulls = Vec<SimplePullRequest>;

/// Only used when getting pull requests in a list.
///
/// If you aren't listing multiple pull requests, please use [`PullRequest`]
/// instead.
///
/// <https://docs.github.com/en/rest/pulls/pulls#list-pull-requests=>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimplePullRequest {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assignee: Option<SimpleUser>,
    pub labels: Vec<Label>,
    pub base: HeadBase,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub head: HeadBase,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub issue_url: String,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<Milestone>,
    pub number: i64,
    pub patch_url: String,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub statuses_url: String,
    pub state: PullRequestState,
    pub locked: bool,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: SimpleUser,
    pub author_association: Association,
    pub auto_merge: Option<AutoMerge>,
}

/// <https://docs.github.com/en/rest/pulls/pulls#get-a-pull-request=>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    pub additions: i64,
    pub changed_files: i64,
    pub comments: i64,
    pub commits: i64,
    pub deletions: i64,
    pub mergeable: Option<bool>,
    // Variants are not documented
    pub mergeable_state: String,
    pub merged: bool,
    pub maintainer_can_modify: bool,
    pub merged_by: Option<SimpleUser>,
    pub review_comments: i64,
    #[serde(flatten)]
    pub shared: SimplePullRequest,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestState {
    #[default]
    Closed,
    Open,
}

pub mod nested {
    use serde::{Deserialize, Serialize};

    use crate::model::{repositories::Repository, user::SimpleUser};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct HeadBase {
        pub label: String,
        #[serde(rename = "ref")]
        pub ref_field: String,
        pub repo: Option<Repository>,
        pub sha: String,
        pub user: SimpleUser,
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Links {
        pub comments: HRef,
        pub commits: HRef,
        pub statuses: HRef,
        pub html: HRef,
        pub issue: HRef,
        pub review_comments: HRef,
        pub review_comment: HRef,
        #[serde(rename = "self")]
        pub self_field: HRef,
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct HRef {
        pub href: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AutoMerge {
        enabled_by: SimpleUser,
        merge_method: String,
        commit_title: String,
        commit_message: String,
    }
}
