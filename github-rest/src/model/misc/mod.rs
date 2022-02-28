pub mod deployments {
    use crate::model::{prelude::*, user::User};

    /// <https://docs.github.com/en/rest/reference/deployments#get-a-deployment>
    #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Deployment {
        pub url: String,
        pub id: usize,
        pub node_id: String,
        pub sha: String,
        #[serde(rename = "ref")]
        pub ref_field: String,
        // unfortunately don't have possible variants so I can't make an enum
        pub task: String,
        pub payload: Value,
        // see above
        pub original_environment: String,
        pub environment: String,
        pub description: Option<String>,
        pub creator: User,
        pub created_at: Value,
        pub updated_at: Value,
        pub statuses_url: String,
        pub repository_url: String,
        pub transient_environment: bool,
        pub production_environment: bool,
    }

    /// <https://docs.github.com/en/rest/reference/deployments#list-deployment-statuses>
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct DeploymentStatus {
        pub url: String,
        pub id: usize,
        pub node_id: String,
        pub state: DeploymentState,
        pub creator: User,
        pub description: Option<String>,
        pub environment: String,
        pub target_url: Option<String>,
        pub created_at: String,
        pub updated_at: String,
        pub deployment_url: String,
        pub repository_url: String,
        pub log_url: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "snake_case")]
    pub enum DeploymentState {
        Pending,
        Success,
        Failure,
        Error,
    }

    /// See this documentation page for a detailed overview of what this struct can contain: <https://docs.github.com/en/developers/github-marketplace/using-the-github-marketplace-api-in-your-app/webhook-events-for-the-github-marketplace-api#github-marketplace-purchase-webhook-payload>
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MarketplacePurchase {
        // TODO: Create enum repr for this field.
        /// Can either be a [`User`] or [`Organization`]
        ///
        /// [`User`]: crate::model::User
        /// [`Organization`]: crate::model::organizations::Organization
        pub account: Value,
        pub billing_cycle: MarketplaceBillingCycle,
        pub unit_count: usize,
        pub on_free_trial: bool,
        pub free_trial_ends_on: String,
        pub next_billing_date: String,
        pub plan: MarketplacePlan,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "snake_case")]
    pub enum MarketplaceBillingCycle {
        Yearly,
        Monthly,
        Nil,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MarketplacePlan {
        pub id: usize,
        pub name: String,
        pub description: String,
        pub monthly_price_in_cents: usize,
        pub yearly_price_in_cents: usize,
        pub price_model: MarketplacePriceModel,
        pub has_free_trial: bool,
        pub unit_name: String,
        pub bullet: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "kebab-case")]
    pub enum MarketplacePriceModel {
        FlatRate,
        PerUnit,
        Free,
    }
}

pub mod events {
    use crate::model::{
        event_types::{macros::repo_origin, Event, RepoEventInfo},
        issues::Label,
        misc::deployments::{Deployment, DeploymentStatus, MarketplacePurchase},
        prelude::*,
        pull_requests::events::nested::Change,
        user::User,
    };

    // TODO: Consider moving deployment events to their own modules.

    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#deployment>
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct DeploymentEvent {
        pub action: DeploymentAction,
        pub deployment: Deployment,
        #[serde(flatten)]
        pub event_info: RepoEventInfo,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "snake_case")]
    pub enum DeploymentStatusAction {
        Created,
    }

    // TODO: Move this to issues, I'm lazy
    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#label>
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LabelEvent {
        pub action: LabelAction,
        pub label: Label,
        pub changes: Option<LabelChanges>,
        #[serde(flatten)]
        pub event_info: RepoEventInfo,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "snake_case")]
    pub enum LabelAction {
        Created,
        Edited,
        Deleted,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LabelChanges {
        pub name: Option<Change>,
        pub color: Option<Change>,
    }

    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#marketplace_purchase>
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MarketplacePurchaseEvent {
        pub action: MarketplacePurchaseAction,
        pub effective_date: String,
        pub sender: User,
        pub marketplace_purchase: MarketplacePurchase,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "snake_case")]
    pub enum MarketplacePurchaseAction {
        Purchased,
        PendingChange,
        PendingChangeCancelled,
        Changed,
        Cancelled,
    }

    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#sponsorship>
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct SponsorshipEvent {
        pub action: SponsorshipAction,
        pub effective_date: String,
        pub changes: SponsorshipChanges,
        pub sender: User,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct SponsorshipChanges {
        pub tier: Option<Change>,
        pub privacy_level: Option<Change>,
    }

    impl Event<'_> for SponsorshipEvent {
        type Origin = User;
    }

    repo_origin!(LabelEvent);
    repo_origin!(DeploymentEvent);
    repo_origin!(DeploymentStatusEvent);
}
