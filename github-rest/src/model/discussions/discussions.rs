use crate::model::{prelude::*, user::User};

// TODO: Complete this (see links below)
// https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#discussion
// https://docs.github.com/en/graphql/guides/using-the-graphql-api-for-discussions#discussion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Discussion {
    pub lock_reason: Option<LockReason>,
    pub repository_url: String,
    pub answer_html_url: Option<String>,
    pub answer_chosen_at: Option<String>,
    pub answer_chosen_by: Option<User>,
    pub id: usize,
    pub node_id: String,
    pub number: usize,
    pub title: String,
    pub user: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum LockReason {
    OffTopic,
    Resolved,
    Spam,
    TooHeated,
}
