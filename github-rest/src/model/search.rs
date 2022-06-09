use crate::model::prelude::*;

/// Descending is the default.
///
/// -Desc variants: Descending order.
/// -Asc variants: Ascending order.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Sort {
    InteractionsDesc,
    InteractionsAsc,
    ReactionsDesc,
    ReactionsAsc,
    #[serde(rename = "reactions-+1")]
    ReactionsThumbsUp,
    #[serde(rename = "reactions--1")]
    ReactionsThumbsDown,
    ReactionsSmile,
    ReactionsTada,
    ReactionsHeart,
    AuthorDateDesc,
    AuthorDateAsc,
    CommitterDateDesc,
    CommitterDateAsc,
    UpdatedDesc,
    UpdatedAsc,
}
