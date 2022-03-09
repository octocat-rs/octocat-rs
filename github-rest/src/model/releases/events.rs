use crate::model::{
    event_types::{macros::repo_origin, RepoEventInfo},
    prelude::*,
    releases::Release,
};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#release>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseEvent {
    pub action: ReleaseAction,
    pub release: Release,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

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

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#create>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEvent {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub ref_type: RefType,
    pub master_branch: String,
    pub description: String,
    pub pusher_type: String,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#delete>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEvent {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub ref_type: RefType,
    pub pusher_type: String,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum RefType {
    Branch,
    Tag,
}

repo_origin!(ReleaseEvent);
repo_origin!(CreateEvent);
repo_origin!(DeleteEvent);
