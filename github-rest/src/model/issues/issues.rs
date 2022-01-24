use super::super::prelude::*;

use crate::model::{
    issues::{milestones::Milestone, nested::PullRequest},
    user::User,
};

pub type Issues = Vec<Issue>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Issue {
    id: i64,
    node_id: String,
    url: String,
    repository_url: String,
    labels_url: String,
    comments_url: String,
    events_url: String,
    html_url: String,
    number: i64,
    state: String,
    title: String,
    body: String,
    user: User,
    labels: Vec<Label>,
    assignee: Option<User>,
    assignees: Vec<User>,
    milestone: Option<Milestone>,
    locked: bool,
    active_lock_reason: Option<String>,
    comments: i64,
    pull_request: Option<PullRequest>,
    closed_at: Option<serde_json::Value>,
    created_at: String,
    updated_at: String,
    closed_by: Option<User>,
    author_association: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Label {
    id: i64,
    node_id: String,
    url: String,
    name: String,
    description: String,
    color: String,
    label_default: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestedTeam {
    id: i64,
    node_id: String,
    url: String,
    html_url: String,
    name: String,
    slug: String,
    description: String,
    privacy: String,
    permission: String,
    members_url: String,
    repositories_url: String,
    parent: Option<serde_json::Value>,
}

pub mod nested {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct PullRequest {
        url: String,
        html_url: String,
        diff_url: String,
        patch_url: String,
    }
}
