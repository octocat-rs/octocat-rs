// TODO: Move this to issues, I'm lazy
pub mod events {
    use crate::model::{event_types::RepoEventInfo, issues::Label, prelude::*, pull_requests::events::nested::Change};

    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#label>
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LabelEvent {
        pub action: LabelAction,
        pub label: Label,
        pub changes: Option<LabelChanges>,
        #[serde(flatten)]
        pub repo_info: RepoEventInfo,
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
}
