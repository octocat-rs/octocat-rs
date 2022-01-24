use super::super::prelude::*;

use crate::model::{issues::Issue, reactions::Reactions, user::User};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueComment {
    pub url: String,
    pub html_url: String,
    pub issue_url: String,
    pub id: i64,
    pub node_id: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub body: String,
    pub reactions: Reactions,
    pub performed_via_github_app: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueCommentEvent {
    action: IssueCommentAction,
    issue: Issue,
    comment: IssueComment,
    sender: User,
}
