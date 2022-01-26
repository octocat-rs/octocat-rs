// TODO: Figure out which events may not originate from a repository.

// Temporary
#![allow(clippy::module_inception)]

pub mod commits;
pub mod event_types;
pub mod issues;
pub mod organizations;
pub mod pull_requests;
pub mod reactions;
pub mod releases;
pub mod repositories;
pub mod user;

pub(crate) mod prelude {
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
    pub use strum::{EnumString, EnumVariantNames};
}
