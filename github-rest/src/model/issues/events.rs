use super::super::prelude::*;

use crate::model::{issues::Issue, repositories::Repository, user::User};

// TODO: Complete this
/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#issues>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueEvent {
    pub action: IssueAction,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IssueAction {
    Opened,
    Edited,
    Deleted,
    Pinned,
    Unpinned,
    Closed,
    Reopened,
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    Locked,
    Unlocked,
    Transferred,
    Milestoned,
    Demilestoned,
}

// TODO: IssueComment
// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#issue_comment>

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}
