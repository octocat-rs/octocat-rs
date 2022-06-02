use crate::model::{prelude::*, reactions::ReactionRollup, user::SimpleUser};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueComment {
    pub url: String,
    pub html_url: String,
    pub issue_url: String,
    pub id: i64,
    pub node_id: String,
    pub user: SimpleUser,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub body: String,
    pub reactions: ReactionRollup,
    pub performed_via_github_app: Value,
}
