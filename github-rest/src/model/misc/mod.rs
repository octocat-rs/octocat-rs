pub mod deployments {
    use crate::model::{organizations::Organization, prelude::*, user::User};

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
        /// Can either be a [`User`] or [`Organization`]
        ///
        /// [`User`]: crate::model::user::User
        /// [`Organization`]: crate::model::organizations::Organization
        #[serde(flatten)]
        pub account: UserOrOrg,
        pub billing_cycle: MarketplaceBillingCycle,
        pub unit_count: usize,
        pub on_free_trial: bool,
        pub free_trial_ends_on: String,
        pub next_billing_date: String,
        pub plan: MarketplacePlan,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum UserOrOrg {
        User(User),
        Organization(Organization),
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

pub mod events;
