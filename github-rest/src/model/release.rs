use super::{Repository, User};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{EnumString, EnumVariantNames};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum ReleaseAction {
    Published,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseEvent {
    pub action: ReleaseAction,
    pub release: Release,
    pub repository: Repository,
    pub sender: User,
}

// Event gets emitted on a tag create (?)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEvent {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub ref_type: String,
    pub master_branch: String,
    pub description: Value,
    pub pusher_type: String,
    pub repository: Repository,
    pub sender: User,
}

// Event gets emitted on a branch delete (?)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEvent {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub ref_type: String,
    pub pusher_type: String,
    pub repository: Repository,
    pub sender: User,
}

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
