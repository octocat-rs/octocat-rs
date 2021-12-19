/// Used in [`Client`] to represent the authorization method
///
/// [`Client`]: crate::github::Client
pub enum Authorization {
    PersonalToken { username: String, token: String },
}
