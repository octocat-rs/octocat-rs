use crate::model::{
    event_types::{macros::repo_origin, Event, RepoEventInfo},
    misc::deployments::{nested::MarketplacePurchase, Deployment, DeploymentStatus},
    prelude::*,
    pull_requests::events::nested::Change,
    user::SimpleUser,
};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#deployment>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentEvent {
    pub action: DeploymentAction,
    pub deployment: Deployment,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentAction {
    Created,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#deployment_status>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentStatusEvent {
    pub action: DeploymentStatusAction,
    pub deployment_status: DeploymentStatus,
    pub deployment: Deployment,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentStatusAction {
    Created,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#marketplace_purchase>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplacePurchaseEvent {
    pub action: MarketplacePurchaseAction,
    pub effective_date: String,
    pub sender: SimpleUser,
    pub marketplace_purchase: MarketplacePurchase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum MarketplacePurchaseAction {
    Purchased,
    PendingChange,
    PendingChangeCancelled,
    Changed,
    Cancelled,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#sponsorship>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SponsorshipEvent {
    pub action: SponsorshipAction,
    pub effective_date: String,
    pub changes: SponsorshipChanges,
    pub sender: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum SponsorshipAction {
    /// `Created` is only triggered after the payment is processed.
    Created,
    Cancelled,
    Edited,
    TierChanged,
    PendingCancellation,
    PendingTierChange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SponsorshipChanges {
    pub tier: Option<Change>,
    pub privacy_level: Option<Change>,
}

impl Event<'_> for SponsorshipEvent {
    type Origin = SimpleUser;
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#meta>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaEvent {
    pub action: MetaAction,
    pub hook_id: usize,
    pub hook: Value,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum MetaAction {
    Deleted,
}

repo_origin!(MetaEvent);
repo_origin!(DeploymentEvent);
repo_origin!(DeploymentStatusEvent);
