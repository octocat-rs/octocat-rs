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
}

pub mod events {
    use crate::model::{
        event_types::{macros::repo_origin, RepoEventInfo},
        issues::Label,
        misc::deployments::{Deployment, DeploymentStatus},
        prelude::*,
        pull_requests::events::nested::Change,
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

    repo_origin!(LabelEvent);
    repo_origin!(DeploymentEvent);
    repo_origin!(DeploymentStatusEvent);
}
