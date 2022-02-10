pub mod events {
    use crate::model::{event_types::RepoEventInfo, prelude::*};

    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#discussion>
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct DiscussionEvent {
        pub action: DiscussionAction,
        // TODO: <https://docs.github.com/en/graphql/guides/using-the-graphql-api-for-discussions#discussion>
        pub discussion: Value,
        #[serde(flatten)]
        pub event_info: RepoEventInfo,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "snake_case")]
    pub enum DiscussionAction {
        Created,
        Edited,
        Deleted,
        Pinned,
        Unpinned,
        Locked,
        Unlocked,
        Transferred,
        CategoryChanged,
        Answered,
        Unanswered,
        Labeled,
        Unlabeled,
    }
}