//! This module contains helper functions for writing API requests.

pub use commits::*;
pub use gists::*;
pub use issues::*;
pub use misc::*;
pub use search::*;
pub use users::*;

mod commits;
mod gists;
mod issues;
mod misc;
mod search;
mod users;
pub(crate) mod util;

/// Prelude mod used for methods
pub mod prelude {
    pub use github_api_octocat::end_points::*;
    #[cfg(not(target_family = "wasm"))]
    pub use reqwest::Body;
    pub use serde::{Deserialize, Serialize};

    pub use crate::{GithubRestError, Requester};

    pub(crate) use crate::methods::user_and_pagination_methods;
}

/// Macro to define methods that take a user and pagination parameters. This
/// expands to:
///
/// ```rust,ignore,does-not-compile
/// pub async fn name<T, A>(
///     client: &T,
///     user: A,
///     params: Option<&Pagination>,
/// ) -> Result<Type, GithubRestError>
/// where
///     T: Requester,
///     A: Into<String>,
/// {
///     client
///         .req::<Pagination, String, Type>(EndPoints::Variant(user.into()), params, None)
///         .await
/// }
macro_rules! user_and_pagination_methods {
    (
        $(
            $(#[$attr:meta])*
            $name:ident, $enum_variant:ty, $return_type:ty
        ),*
    ) => {
        paste::paste! {
            $(
                $(#[$attr])*
                pub async fn $name<T, A>(
                    client: &T,
                    user: A,
                    params: Option<&Pagination>,
                ) -> Result<$return_type, crate::GithubRestError>
                where
                    T: crate::Requester,
                    A: Into<String>,
                {
                    client
                        .req::<Pagination, String, $return_type>($enum_variant(user.into()), params, None)
                        .await
                }
            )*
        }
    }
}

pub(crate) use user_and_pagination_methods;
