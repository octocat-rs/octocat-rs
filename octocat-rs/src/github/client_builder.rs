//! Contains a builder for [`Client`].

use std::{ffi::OsStr, fs::File, io::prelude::*, path::Path};

use anyhow::{Error, Result};

use crate::github::{handler::EventHandler, util::*, Client, DefaultEventHandler};

/// A builder for [`Client`]
pub struct ClientBuilder<T>
where
    T: std::fmt::Debug + EventHandler<GitHubClient = Client<T>> + Send + Sync + 'static,
{
    handler: Option<T>,
    auth: Option<Authorization>,
    user_agent: Option<String>,
    payload_size: Option<u64>,
}

impl<T> ClientBuilder<T>
where
    T: std::fmt::Debug + EventHandler<GitHubClient = Client<T>> + Send + Sync + 'static,
{
    /// Creates a new [`ClientBuilder`]
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds an [`EventHandler`] to the current builder.
    pub fn event_handler(mut self, event_handler: T) -> Self {
        self.handler = Some(event_handler);
        self
    }

    /// Sets the maximum payload size that the listener can receive from GitHub
    /// in MiB. Default: 8.
    pub fn payload_size(mut self, size: u64) -> Self {
        self.payload_size = Some(size);
        self
    }

    /// Sets a custom user agent for your application. Default is "Octocat-rs".
    ///
    /// See also: [`HttpClient::set_ua`]
    ///
    /// [`HttpClient::set_ua`]: crate::github::HttpClient::set_ua
    pub fn user_agent<V: Into<String>>(mut self, user_agent: V) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Adds an [`Authorization`] instance to the current builder using input
    /// from a file.
    pub fn credentials_file<P: AsRef<Path>>(self, file: P) -> Self {
        let mut f = File::open(file).expect("ClientBuilder: Opening authorization file");
        let mut contents = "".to_owned();

        f.read_to_string(&mut contents)
            .expect("ClientBuilder: Reading authorization file");

        let auth: Option<Authorization> = Some(
            toml::from_str::<OctocatConfig>(contents.as_str())
                .expect("ClientBuilder: Parsing authorization file")
                .to_personal_auth(),
        );

        self.set_auth(auth)
    }

    /// Adds an [`Authorization`] instance to the current builder using input
    /// from an environment variable.
    pub fn credentials_env_var<K: AsRef<OsStr>>(self, username_var: K, token_var: K) -> Self {
        let username = match std::env::var(username_var) {
            Ok(u) => u,
            Err(e) => panic!("{}", e),
        };

        let token = match std::env::var(token_var) {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        };

        let auth = Some(Authorization::PersonalToken { username, token });

        self.set_auth(auth)
    }

    /// Adds an [`Authorization`] instance to the current builder.
    pub fn personal_auth<V: Into<String>>(self, username: V, token: V) -> Self {
        let auth = Some(Authorization::PersonalToken {
            username: username.into(),
            token: token.into(),
        });

        self.set_auth(auth)
    }

    fn set_auth(mut self, auth: Option<Authorization>) -> Self {
        self.auth = auth;
        self
    }

    /// Builds the current builder. In other words, this turns a
    /// [`ClientBuilder`] into a [`Client`]. Requires a handler to be set.
    pub fn build(self) -> Result<Client<T>> {
        if self.handler.is_none() {
            return Err(Error::from(BuildError::NoHandler));
        }

        Ok(Client::new(
            self.handler.unwrap(),
            self.auth,
            self.user_agent,
            self.payload_size,
        ))
    }
}

impl ClientBuilder<DefaultEventHandler> {
    /// Returns the default implementation of [`Client`]
    pub fn build_unconfigured() -> Client<DefaultEventHandler> {
        Client::default()
    }

    /// For building the current builder without setting a handler.
    ///
    /// Requires T to be set to [`DefaultEventHandler`].
    pub fn build_no_handler(self) -> Result<Client<DefaultEventHandler>> {
        Ok(Client::new(
            DefaultEventHandler::new(),
            self.auth,
            self.user_agent,
            self.payload_size,
        ))
    }
}

impl<T> Default for ClientBuilder<T>
where
    T: std::fmt::Debug + EventHandler<GitHubClient = Client<T>> + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            handler: None,
            auth: None,
            payload_size: None,
            user_agent: None,
        }
    }
}
