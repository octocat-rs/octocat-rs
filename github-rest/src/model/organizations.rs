use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub avatar_url: String,
    pub description: Option<String>,
    pub events_url: String,
    pub hooks_url: String,
    pub id: i64,
    pub issues_url: String,
    pub login: String,
    pub members_url: String,
    pub node_id: String,
    pub public_members_url: String,
    pub repos_url: String,
    pub url: String,
}
