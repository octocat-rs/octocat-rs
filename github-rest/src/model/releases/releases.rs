use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::user::User;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i64,
    pub author: User,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Value>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
}
