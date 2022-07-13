use crate::model::{prelude::*, user::SimpleUser};
use std::collections::HashMap;

/// <https://docs.github.com/en/rest/gists/gists#list-gists-for-the-authenticated-user=>
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimpleGist {
    pub id: String,
    pub node_id: String,
    pub url: String,
    pub forks_url: String,
    pub commits_url: String,
    pub git_pull_url: String,
    pub git_push_url: String,
    pub html_url: String,
    pub comments_url: String,
    pub public: bool,
    pub description: Option<String>,
    pub comments: usize,
    pub user: Option<SimpleUser>,
    pub files: HashMap<String, File>,
    pub created_at: String,
    pub updated_at: String,
}

/// <https://docs.github.com/en/rest/gists/gists#get-a-gist=>
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gist {
    pub owner: SimpleUser,
    pub truncated: bool,
    #[serde(flatten)]
    pub shared: SimpleGist,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    pub filename: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub language: Option<String>,
    pub raw_url: String,
    pub size: i64,
    pub content: Option<String>,
}
