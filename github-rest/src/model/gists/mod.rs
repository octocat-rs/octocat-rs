use std::collections::HashMap;

use crate::model::{prelude::*, user::User};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gist {
    pub url: String,
    pub forks_url: String,
    pub commits_url: String,
    pub id: String,
    pub node_id: String,
    pub git_pull_url: String,
    pub git_push_url: String,
    pub html_url: String,
    pub files: HashMap<String, File>,
    pub public: bool,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
    pub comments: i64,
    pub user: Value,
    pub comments_url: String,
    pub owner: User,
    pub truncated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub filename: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub language: Option<String>,
    pub raw_url: String,
    pub size: i64,
}
