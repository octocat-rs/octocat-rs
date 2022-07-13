use crate::model::{apps::events::nested::RepoInfo, prelude::*, user::SimpleUser};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#installation>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallationEvent {
    pub action: InstallationAction,
    pub repositories: Vec<RepoInfo>,
    pub installation: Value,
    pub sender: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum InstallationAction {
    Created,
    Deleted,
    Suspend,
    Unsuspend,
    NewPermissionsAccepted,
}

pub mod nested {
    use crate::model::prelude::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RepoInfo {
        pub id: usize,
        pub node: String,
        pub name: String,
        pub full_name: String,
        pub private: bool,
    }
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#installation_repositories>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallationRepositoriesEvent {
    pub action: InstallationRepositoriesAction,
    pub repository_selection: RepositorySelection,
    pub repositories_added: Vec<RepoInfo>,
    pub repositories_removed: Vec<RepoInfo>,
    pub installation: Value,
    pub sender: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum InstallationRepositoriesAction {
    Added,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum RepositorySelection {
    Selected,
    All,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#github_app_authorization>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppAuthorizationEvent {
    pub action: String,
    pub sender: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum AppAuthorizationAction {
    Revoked,
}
