use crate::model::{
    event_types::{macros::repo_origin, RepoEventInfo},
    issues::{comments::IssueComment, Issue, Label},
    prelude::*,
    pull_requests::events::nested::{Change, IssueChanges},
};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#issues>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueEvent {
    pub action: IssueAction,
    pub issue: Issue,
    pub changes: Option<IssueChanges>,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

as_ref_and_deref!(IssueEvent, RepoEventInfo, event_info);

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
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

as_ref_and_deref!(IssueCommentEvent, RepoEventInfo, event_info);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentChanges {
    pub body: Option<Change>,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#label>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelEvent {
    pub action: LabelAction,
    pub label: Label,
    pub changes: Option<LabelChanges>,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

as_ref_and_deref!(LabelEvent, RepoEventInfo, event_info);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum LabelAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelChanges {
    pub name: Option<Change>,
    pub color: Option<Change>,
}

repo_origin!(IssueEvent);
repo_origin!(LabelEvent);
repo_origin!(IssueCommentEvent);
