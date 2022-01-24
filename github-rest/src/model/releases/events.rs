use super::super::prelude::*;

use crate::model::{releases::Release, repositories::Repository, user::User};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#release>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseEvent {
    pub action: nested::ReleaseAction,
    pub release: Release,
    pub repository: Repository,
    pub sender: User,
}

pub mod nested {
    use super::super::super::prelude::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[strum(serialize_all = "snake_case")]
    pub enum ReleaseAction {
        Published,
        Unpublished,
        Created,
        Edited,
        Deleted,
        #[strum(serialize = "prereleased")]
        PreReleased,
        Released,
    }
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#create>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEvent {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub ref_type: RefType,
    pub master_branch: String,
    pub description: String,
    pub pusher_type: String,
    pub repository: Repository,
    pub sender: User,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#delete>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEvent {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub ref_type: RefType,
    pub pusher_type: String,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum RefType {
    Branch,
    Tag,
}
