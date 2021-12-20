use anyhow::{Error, Result};

use crate::github::{handler::EventHandler, util::Authorization, BuildError, Client, DefaultEventHandler};

/// A builder for [`Client`]
pub enum ClientBuilder<T>
where
    T: std::fmt::Debug + EventHandler,
{
    Unconfigured,
    Configured {
        handler: Option<T>,
        auth: Option<Authorization>,
    },
}

impl<T> ClientBuilder<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    /// Creates a new [`ClientBuilder`]
    pub fn new() -> Self {
        Self::Unconfigured
    }

    /// Adds an [`EventHandler`] to the current builder.
    pub fn event_handler(self, event_handler: T) -> Self {
        match self {
            ClientBuilder::Unconfigured => ClientBuilder::Configured {
                handler: Some(event_handler),
                auth: None,
            },
            ClientBuilder::Configured { auth, .. } => ClientBuilder::Configured {
                auth,
                handler: Some(event_handler),
            },
        }
    }

    /// Adds an [`Authorization`] instance to the current builder using input
    /// from a file.
    #[allow(unused_variables)]
    pub fn credentials_file(self, file: &str) -> Self {
        todo!("This logic is a little annoying so I won't write it just yet")
    }

    // TODO: Minimize code duplication, other cleanup

    /// Adds an [`Authorization`] instance to the current builder using input
    /// from an environment variable.
    #[allow(unused_variables)]
    pub fn credentials_env_var(self, username_var: &str, token_var: &str) -> Self {
        let username = match std::env::var(username_var) {
            Ok(u) => u,
            Err(e) => panic!("{}", e),
        };

        let token = match std::env::var(token_var) {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        };

        let auth = Some(Authorization::PersonalToken { username, token });

        match self {
            ClientBuilder::Unconfigured => ClientBuilder::Configured { handler: None, auth },
            ClientBuilder::Configured { handler, .. } => ClientBuilder::Configured { handler, auth },
        }
    }

    /// Adds an [`Authorization`] instance to the current builder.
    pub fn personal_auth(self, username: &str, token: &str) -> Self {
        let auth = Some(Authorization::PersonalToken {
            username: username.to_owned(),
            token: token.to_owned(),
        });
        match self {
            ClientBuilder::Unconfigured => ClientBuilder::Configured { handler: None, auth },
            ClientBuilder::Configured { handler, .. } => ClientBuilder::Configured { handler, auth },
        }
    }

    /// Builds the current builder. In other words, this turns a
    /// [`ClientBuilder`] into a [`Client`]. Requires a handler to be set.
    pub fn build(self) -> Result<Client<T>> {
        match self {
            ClientBuilder::Unconfigured => Err(Error::from(BuildError::NotConfigured)),
            ClientBuilder::Configured { handler, auth } => {
                let handler = handler.unwrap();
                let auth = auth.unwrap_or_default();
                Ok(Client::<T>::new(handler, auth))
            }
        }
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
        match self {
            ClientBuilder::Unconfigured => Err(Error::from(BuildError::NotConfigured)),
            ClientBuilder::Configured { auth, .. } => Ok(Client::default().set_auth(auth.unwrap())),
        }
    }
}

impl<T> Default for ClientBuilder<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    fn default() -> Self {
        Self::new()
    }
}
