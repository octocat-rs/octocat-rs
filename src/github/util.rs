use thiserror::Error;

/// Used in [`Client`] to represent the authorization method
///
/// [`Client`]: crate::github::Client
pub enum Authorization {
    PersonalToken { username: String, token: String },
}

impl Default for Authorization {
    fn default() -> Self {
        Authorization::PersonalToken {
            username: String::new(),
            token: String::new(),
        }
    }
}

/// Used to represent errors when building a [`ClientBuilder`]
///
/// [`ClientBuilder`]: crate::github::ClientBuilder
#[derive(Error, Debug)]
pub enum BuildError {
    #[error("Call build_unconfigured instead!")]
    NotConfigured,
}
