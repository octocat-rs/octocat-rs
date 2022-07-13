use crate::model::{
    discussions::{Discussion, DiscussionComment},
    event_types::{macros::repo_origin, RepoEventInfo},
    prelude::*,
};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#discussion>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscussionEvent {
    pub action: DiscussionAction,
    pub discussion: Discussion,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionAction {
    Created,
    Edited,
    Deleted,
    Pinned,
    Unpinned,
    Locked,
    Unlocked,
    Transferred,
    CategoryChanged,
    Answered,
    Unanswered,
    Labeled,
    Unlabeled,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#discussion_comment>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscussionCommentEvent {
    pub action: DiscussionCommentAction,
    pub comment: DiscussionComment,
    pub discussion: Discussion,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionCommentAction {
    Created,
    Edited,
    Deleted,
}

repo_origin!(DiscussionEvent);
repo_origin!(DiscussionCommentEvent);
