use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

use crate::model::{pull_requests::PullRequest, repositories::Repository, user::User};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: i64,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum PullRequestAction {
    Opened,
    // = merged/closed
    Closed,
    Reopened,
}
