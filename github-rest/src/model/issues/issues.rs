use crate::model::{
    commits::association::Association,
    issues::{
        milestones::Milestone,
        nested::{StringOrLabel, *},
    },
    prelude::*,
    user::SimpleUser,
};

pub type Issues = Vec<Issue>;

/// <https://docs.github.com/en/rest/issues/issues#get-an-issue=>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    pub assignee: Option<SimpleUser>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub labels: Vec<StringOrLabel>,
    pub labels_url: String,
    pub milestone: Option<Milestone>,
    pub number: i64,
    pub repository_url: String,
    pub state: IssueState,
    pub locked: bool,
    pub title: String,
    pub url: String,
    pub user: Option<SimpleUser>,
    pub author_association: Association,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

pub mod nested {
    use crate::model::{issues::Label, prelude::*};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum IssueState {
        Open,
        Closed,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum StringOrLabel {
        String(String),
        Label(Label),
    }
}
