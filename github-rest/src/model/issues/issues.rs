use crate::{
    methods::IssueState,
    model::{
        commits::association::Association,
        issues::{milestones::Milestone, nested::PullRequest},
        prelude::*,
        user::SimpleUser,
    },
};

pub type Issues = Vec<Issue>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Issue {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub number: i64,
    pub state: IssueState,
    pub title: String,
    pub body: Option<String>,
    pub user: Option<SimpleUser>,
    pub labels: Vec<Label>,
    pub assignee: Option<SimpleUser>,
    pub assignees: Option<Vec<SimpleUser>>,
    pub milestone: Option<Milestone>,
    pub locked: bool,
    pub active_lock_reason: Option<String>,
    pub comments: i64,
    pub pull_request: Option<PullRequest>,
    pub closed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_by: Option<SimpleUser>,
    pub author_association: Association,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    // TODO: Check if this is really optional
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RequestedTeam {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub privacy: String,
    pub permission: String,
    pub members_url: String,
    pub repositories_url: String,
    pub parent: Option<serde_json::Value>,
}

pub mod nested {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct PullRequest {
        pub url: String,
        pub html_url: String,
        pub diff_url: String,
        pub patch_url: String,
    }
}
