use super::super::prelude::*;

use crate::model::{pull_requests::PullRequest, repositories::Repository, user::User};

// TODO: Complete this
/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#pull_request>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: i64,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
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

// TODO: PullRequestReview & PullRequestReviewComment
// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#pull_request_review>
// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#pull_request_review_comment>
