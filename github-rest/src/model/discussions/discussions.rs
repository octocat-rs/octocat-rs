use crate::model::{commits::association::Association, prelude::*, user::User};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Discussion {
    pub lock_reason: Option<LockReason>,
    pub repository_url: String,
    pub answer_html_url: Option<String>,
    pub answer_chosen_at: Option<String>,
    pub answer_chosen_by: Option<User>,
    pub html_url: String,
    pub id: usize,
    pub node_id: String,
    pub number: usize,
    pub title: String,
    pub user: User,
    pub state: String,
    pub locked: bool,
    pub comments: usize,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: Association,
    pub active_lock_reason: Option<Value>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum LockReason {
    OffTopic,
    Resolved,
    Spam,
    TooHeated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscussionComment {
    pub id: usize,
    pub node_id: String,
    pub html_url: String,
    pub parent_id: Option<Value>,
    pub child_comment_count: usize,
    pub repository_url: String,
    pub discussion_id: usize,
    pub author_association: Association,
    pub user: User,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub body: String,
}
