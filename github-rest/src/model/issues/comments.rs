use crate::model::{commits::association::Association, prelude::*, reactions::ReactionRollup, user::SimpleUser};

/// <https://docs.github.com/en/rest/issues/comments#get-an-issue-comment=>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueComment {
    pub id: i64,
    pub node_id: String,
    pub html_url: String,
    pub issue_url: String,
    pub author_association: Association,
    pub user: Option<SimpleUser>,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub reactions: Option<ReactionRollup>,
}
