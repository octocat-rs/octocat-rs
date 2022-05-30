use crate::model::{prelude::*, user::SimpleUser};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub path: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
    pub html_url: String,
    pub badge_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBuild {
    pub id: usize,
    pub status: String,
    pub error: Value,
    pub pusher: SimpleUser,
    pub commit: String,
    pub duration: usize,
    pub created_at: String,
    pub updated_at: String,
}
