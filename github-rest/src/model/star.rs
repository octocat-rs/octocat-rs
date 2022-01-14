use super::{Repository, User};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum StarAction {
    Created,
    Deleted,
}
//x-github-event=star
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarEvent {
    pub action: StarAction,
    pub repository: Repository,
    pub sender: User,
}
