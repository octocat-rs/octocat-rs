pub mod events;

pub mod deployments {
    use crate::model::{misc::deployments::nested::*, prelude::*, user::SimpleUser};

    /// <https://docs.github.com/en/rest/deployments/deployments#get-a-deployment=>
    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Deployment {
        pub id: i64,
        pub node_id: String,
        pub sha: String,
        #[serde(rename = "ref")]
        pub ref_field: String,
        pub task: String,
        pub environment: String,
        pub creator: Option<SimpleUser>,
        /// No example given by GitHub's docs. Good luck.
        pub payload: Value,
        pub description: Option<String>,
        pub statuses_url: String,
        pub repository_url: String,
        pub url: String,
        pub created_at: String,
        pub updated_at: String,
    }

    /// <https://docs.github.com/en/rest/deployments/statuses#get-a-deployment-status=>
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DeploymentStatus {
        pub id: i64,
        pub node_id: String,
        pub state: DeploymentState,
        pub creator: Option<SimpleUser>,
        pub description: String,
        pub deployment_url: String,
        pub target_url: String,
        pub repository_url: String,
        pub url: String,
        pub created_at: String,
        pub updated_at: String,
    }

    pub mod nested {
        use crate::model::{organizations::SimpleOrganization, prelude::*, user::SimpleUser};

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
        #[serde(rename_all = "snake_case")]
        pub enum DeploymentState {
            Error,
            Failure,
            Inactive,
            Pending,
            Success,
            Queued,
            InProgress,
        }

        /// See this documentation page for a detailed overview of what this struct can contain: <https://docs.github.com/en/developers/github-marketplace/using-the-github-marketplace-api-in-your-app/webhook-events-for-the-github-marketplace-api#github-marketplace-purchase-webhook-payload>
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum UserOrOrg {
            User(SimpleUser),
            Organization(SimpleOrganization),
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
        #[serde(rename_all = "snake_case")]
        pub enum MarketplaceBillingCycle {
            Yearly,
            Monthly,
            /// Account owner has a free GitHub plan and has purchased a free
            /// GitHub Marketplace plan.
            Nil,
        }

        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
        #[serde(rename_all = "kebab-case")]
        pub enum MarketplacePriceModel {
            FlatRate,
            PerUnit,
            Free,
        }
    }
}
