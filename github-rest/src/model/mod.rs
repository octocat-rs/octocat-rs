//! This module contains all API request/response types currently implemented.
//! These include:
//! * Webhook payloads
//!     - Webhook payloads can be found at `./<directory>/events.rs`
//! * Request bodies
//! * Response bodies
//!
//! To create new structs please use the following tool: https://transform.tools/json-to-rust-serde

// Temporary
#![allow(clippy::module_inception)]

pub mod apps;
pub mod commits;
pub mod discussions;
pub mod event_types;
pub mod gists;
pub mod issues;
pub mod keys;
pub mod misc;
pub mod organizations;
pub mod pull_requests;
pub mod reactions;
pub mod releases;
pub mod repositories;
pub mod user;

macro_rules! as_ref_and_deref {
    ($from:ty, $to:ty, $field:ident) => {
        impl AsRef<$to> for $from {
            fn as_ref(&self) -> &$to {
                &self.$field
            }
        }

        impl std::ops::Deref for $from {
            type Target = $to;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }
    };
}

pub(crate) use as_ref_and_deref;

pub(crate) mod prelude {
    pub(crate) use super::as_ref_and_deref;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
    pub use strum::{EnumString, EnumVariantNames};
}
