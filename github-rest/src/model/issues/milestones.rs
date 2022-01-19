use serde::{Deserialize, Serialize};

use crate::model::user::User;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Milestone {
    url: String,
    html_url: String,
    labels_url: String,
    id: i64,
    node_id: String,
    number: i64,
    state: String,
    title: String,
    description: String,
    creator: User,
    open_issues: i64,
    closed_issues: i64,
    created_at: String,
    updated_at: String,
    closed_at: String,
    due_on: String,
}
