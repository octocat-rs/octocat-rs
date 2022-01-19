use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Association {
    Collaborator,
    Contributor,
    FirstTimer,
    FirstTimeContributor,
    Mannequin,
    Member,
    None,
    Owner,
}

impl Default for Association {
    fn default() -> Self {
        Association::None
    }
}
