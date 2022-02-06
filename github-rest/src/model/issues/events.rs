use crate::model::{
    event_types::macros::repo_origin,
    issues::{comments::IssueComment, Issue},
    organizations::Organization,
    prelude::*,
    pull_requests::events::nested::{Change, IssueChanges},
    repositories::Repository,
    user::User,
};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#issues>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueEvent {
    pub action: IssueAction,
    pub issue: Issue,
    pub repository: Repository,
    pub organization: Option<Organization>,
    // Couldn't find any example in the docs for this
    pub installation: Option<Value>,
    pub sender: User,
    pub changes: Option<IssueChanges>,
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

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#issue_comment>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCommentEvent {
    pub action: IssueAction,
    pub changes: Option<CommentChanges>,
    pub comment: IssueComment,
    pub repository: Repository,
    pub organization: Option<Organization>,
    // NOTE: Both of these don't appear to have a set structure, hopefully further testing will allow us to weed out
    // possible complications.
    pub installation: Value,
    pub sender: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}

// TODO: Move this to some type of shared module as `PullRequestReviewEvent`
// uses it
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentChanges {
    body: Option<Change>,
}

repo_origin!(IssueEvent);
repo_origin!(IssueCommentEvent);
