use serde::{Deserialize, Serialize};

use super::user::User;
use crate::methods::Role;

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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AddToOrgResponse {
    pub url: String,
    pub state: String,
    pub role: Role,
    pub organization_url: String,
    pub organization: Organization,
    pub user: User,
}
