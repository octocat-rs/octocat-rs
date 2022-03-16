//! This module contains helper functions for writing API requests.

pub use commits::*;
pub use issues::*;
pub use misc::*;
pub use users::*;

mod commits;
mod issues;
mod misc;
mod users;
pub(crate) mod util;

/// Prelude mod used for methods
pub mod prelude {
    pub use github_api_octocat::end_points::*;
    #[cfg(not(target_family = "wasm"))]
    pub use reqwest::Body;
    pub use serde::{Deserialize, Serialize};

    pub use crate::{GithubRestError, Requester};
}
