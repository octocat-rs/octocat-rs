use crate::model::{
    event_types::{macros::org_origin, OrgEventInfo},
    organizations::{
        events::nested::{MembershipScope, TeamChanges},
        Team,
    },
    prelude::*,
    user::User,
};

/// The invitation and membership fields are mutually exclusive.
///
/// [Read more](https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#organization)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationEvent {
    pub action: OrganizationAction,
    pub invitation: Option<Value>,
    pub membership: Option<Value>,
    #[serde(flatten)]
    pub event_info: OrgEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationAction {
    Deleted,
    Renamed,
    MemberAdded,
    MemberRemoved,
    MemberInvited,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#team>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamEvent {
    pub action: TeamAction,
    pub team: Team,
    pub changes: TeamChanges,
    #[serde(flatten)]
    pub event_info: OrgEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum TeamAction {
    Created,
    Deleted,
    Edited,
    AddedToRepository,
    RemovedFromRepository,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#membership>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MembershipEvent {
    pub action: MembershipAction,
    pub scope: MembershipScope,
    pub member: User,
    pub team: Team,
    #[serde(flatten)]
    pub event_info: OrgEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum MembershipAction {
    Added,
    Removed,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#org_block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgBlockEvent {
    pub action: OrgBlockAction,
    pub blocked_user: User,
    #[serde(flatten)]
    pub event_info: OrgEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum OrgBlockAction {
    Blocked,
    Unblocked,
}

pub mod nested {
    use crate::model::{prelude::*, pull_requests::events::nested::Change};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "snake_case")]
    pub enum MembershipScope {
        Team,
    }

    // TODO: Complete this
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TeamChanges {
        pub description: Option<Change>,
        pub name: Option<Change>,
        pub privacy: Option<Change>,
    }
}

org_origin!(OrganizationEvent);
org_origin!(MembershipEvent);
org_origin!(OrgBlockEvent);
org_origin!(TeamEvent);
