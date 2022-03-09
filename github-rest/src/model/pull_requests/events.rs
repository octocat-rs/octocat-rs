use crate::model::{
    event_types::{macros::repo_origin, RepoEventInfo},
    issues::{
        comments::IssueComment,
        events::{CommentChanges, IssueCommentAction},
    },
    prelude::*,
    pull_requests::{events::nested::IssueChanges, PullRequest},
};

// TODO: Convenience method to get changes if the action is `Edited`.
/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#pull_request>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: i64,
    pub pull_request: PullRequest,
    pub changes: Option<IssueChanges>,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestAction {
    Assigned,
    AutoMergeDisabled,
    AutoMergeEnabled,
    // = merged/closed
    Closed,
    ConvertedToDraft,
    Edited,
    Labeled,
    Locked,
    Opened,
    ReadyForReview,
    Reopened,
    ReviewRequested,
    ReviewRequestRemoved,
    Synchronize,
    Unassigned,
    Unlabeled,
    Unlocked,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#pull_request_review>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReviewEvent {
    pub action: PullRequestAction,
    pub changes: Option<CommentChanges>,
    pub pull_request: PullRequest,
    pub review: Value,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum PullRequestReviewAction {
    Submitted,
    Edited,
    Dismissed,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#pull_request_review_comment>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReviewCommentEvent {
    pub action: IssueCommentAction,
    pub changes: Option<CommentChanges>,
    pub pull_request: PullRequest,
    pub comment: IssueComment,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

pub mod nested {
    use crate::model::prelude::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct IssueChanges {
        pub title: Option<Change>,
        pub body: Option<Change>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Change {
        pub from: String,
    }
}

repo_origin!(PullRequestEvent);
repo_origin!(PullRequestReviewEvent);
repo_origin!(PullRequestReviewCommentEvent);
