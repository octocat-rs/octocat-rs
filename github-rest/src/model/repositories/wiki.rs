use crate::model::prelude::*;
pub mod events {
    use crate::model::{prelude::*, repositories::Repository, user::User};

    use super::Page;

    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#gollum>
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct GollumEvent {
        pub pages: Vec<Page>,
        pub repository: Repository,
        pub sender: User,
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub page_name: String,
    pub title: String,
    pub summary: Value,
    pub action: String,
    pub sha: String,
    pub html_url: String,
}
