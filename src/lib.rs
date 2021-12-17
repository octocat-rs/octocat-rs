//! A GitHub API client written in Rust.
//!
//! Getting started? Take a look at the [examples](https://github.com/proudmuslim-dev/octocat-rs/tree/main/examples) folder in the project repository!
//! If you want to get started as quickly as possible, here are some pointers to
//! get you on your way:
//!
//! * [`GitHubPersonalClient`]
//!    - For authenticating with your username + personal access token.
//! * [`GitHubSsoClient`]
//!    - For accessing organizations that enforce SAML SSO.
//! * [`GitHubApplication`]
//!     - This trait is implemented by all clients, and can be used if you want
//!       to build your own from scratch using Octocat.
//!
//! [`GitHubPersonalClient`]: crate::github::GitHubPersonalClient
//! [`GitHubSsoClient`]: crate::github::GitHubSsoClient
//! [`GitHubApplication`]: crate::github::GitHubApplication

pub mod github;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
