use crate::model::{prelude::*, user::SimpleUser};

/// <https://docs.github.com/en/rest/issues/milestones#get-a-milestone=>
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Milestone {
    pub closed_issues: i64,
    pub creator: Option<SimpleUser>,
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub closed_at: Option<String>,
    pub id: i64,
    pub node_id: String,
    pub labels_url: String,
    pub html_url: String,
    pub number: i64,
    pub open_issues: i64,
    pub state: MilestoneState,
    pub title: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneState {
    #[default]
    Open,
    Closed,
}
