use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Used to represent the default Octocat configuration file format.
///
/// ```toml
/// username = "USERNAME"
/// access_token = "TOKEN"
/// ```
#[derive(Serialize, Deserialize)]
pub struct OctocatConfig {
    username: String,
    access_token: String,
}

impl OctocatConfig {
    /// Converts the current [`OctocatConfig`] instance into an
    /// [`Authorization`] instance.
    pub fn to_personal_auth(self) -> Authorization {
        Authorization::PersonalToken {
            username: self.username,
            token: self.access_token,
        }
    }
}

/// Used in [`Client`] to represent the authorization method
///
/// [`Client`]: crate::github::Client
#[derive(Clone)]
pub enum Authorization {
    PersonalToken { username: String, token: String },
}

impl Default for Authorization {
    fn default() -> Self {
        Authorization::PersonalToken {
            username: "".to_owned(),
            token: "".to_owned(),
        }
    }
}

/// Used to represent errors when building a [`ClientBuilder`]
///
/// [`ClientBuilder`]: crate::github::ClientBuilder
#[derive(Error, Debug)]
pub enum BuildError {
    #[error("Call build_no_handler instead!")]
    NoHandler,
}
