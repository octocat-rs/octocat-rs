use crate::model::prelude::*;
pub mod events {
    use crate::model::{
        event_types::{macros::repo_origin, RepoEventInfo},
        prelude::*,
    };

    use super::Page;

    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#gollum>
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct GollumEvent {
        pub pages: Vec<Page>,
        #[serde(flatten)]
        pub event_info: RepoEventInfo,
    }

    repo_origin!(GollumEvent);
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    pub page_name: String,
    pub title: String,
    pub summary: Value,
    pub action: String,
    pub sha: String,
    pub html_url: String,
}
