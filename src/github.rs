use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Copy, Clone)]
pub enum AuthMethod {
    OAuthToken,
    Sso,
}

// TODO: Work out how requests will be handled; will probably be done with some sort of util module

/// A trait to be implemented by you, the user.
#[async_trait]
pub trait GitHubApplication {
    /// Basic username + OAuth token authentication.
    fn new(username: &str, token: &str) -> Self;

    /// For accessing organizations that enforce SAML SSO with a personal access token.
    ///
    /// Further reading: <https://docs.github.com/en/rest/overview/other-authentication-methods#authenticating-for-saml-sso>
    fn new_with_sso(token: &str) -> Self;

    /// Helper function for getting the current authorization method.
    fn current_auth_method(&self) -> AuthMethod;

    /// The code that is run when your application starts. Called by [`start`].
    ///
    /// [`start`]: GitHubApplication::start
    async fn run(&self) -> Result<()>;

    // TODO: Settings interface
    async fn start(&self) -> Result<()> {
        // TODO: Proper logging etc
        self.run().await
    }
}
