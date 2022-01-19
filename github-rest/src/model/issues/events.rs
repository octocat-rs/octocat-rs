use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

use crate::model::{issues::Issue, repositories::Repository, user::User};

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
    Closed,
}
