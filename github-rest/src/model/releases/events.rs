use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::{releases::Release, repositories::Repository, user::User};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseEvent {
    pub action: nested::ReleaseAction,
    pub release: Release,
    pub repository: Repository,
    pub sender: User,
}

pub mod nested {
    use serde::{Deserialize, Serialize};
    use strum::{EnumString, EnumVariantNames};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[strum(serialize_all = "snake_case")]
    pub enum ReleaseAction {
        Published,
    }
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
