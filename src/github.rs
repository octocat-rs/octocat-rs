use std::{panic, process};

/// An API client for github.
#[allow(dead_code)]
pub struct GitHub {
    api_key: String,
    username: Option<String>,
    auth_method: AuthMethod,
}

enum AuthMethod {
    OAuthToken,
    SSO
}

impl GitHub {
    /// Basic username + OAuth token authentication.
    pub fn new(username: &str, token: &str) -> Self {
        Self {
            api_key: token.to_owned(),
            username: Some(username.to_owned()),
            auth_method: AuthMethod::OAuthToken
        }
    }

    /// For accessing organizations that enforce SAML SSO with a personal access token.
    ///
    /// Further reading: <https://docs.github.com/en/rest/overview/other-authentication-methods#authenticating-for-saml-sso>
    pub fn new_with_sso(token: &str) -> Self {
        Self {
            api_key: token.to_owned(),
            username: None,
            auth_method: AuthMethod::SSO
        }
    }

    pub async fn run(&self) {
        pretty_env_logger::init();

        panic::set_hook(Box::new(|msg| {
            match msg.payload().downcast_ref::<&str>() {
                Some(msg) => error!("Panicked at: {}", msg),
                _ => error!("Error occurred")
            }

            if let Some(loc) = msg.location() { error!("Location: {}:{}:{}", loc.file(), loc.line(), loc.column()) }

            process::exit(1);
        }));

        todo!()
    }
}