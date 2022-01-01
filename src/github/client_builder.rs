use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use anyhow::{Error, Result};

use crate::github::{handler::EventHandler, util::*, Client, DefaultEventHandler};

/// A builder for [`Client`]
pub enum ClientBuilder<T>
where
    T: std::fmt::Debug + EventHandler + Send,
{
    Unconfigured,
    Configured {
        handler: Option<T>,
        auth: Option<Authorization>,
        user_agent: Option<String>,
    },
}

// TODO: Figure out how to make this better, it's garbage...
// Might move it to a struct since everything would be much cleaner that way

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
                user_agent: None,
            },
            ClientBuilder::Configured { auth, user_agent, .. } => ClientBuilder::Configured {
                handler: Some(event_handler),
                auth,
                user_agent,
            },
        }
    }

    /// Sets a custom user agent for your application. Default is "Octocat-rs".
    ///
    /// See also: [`HttpClient::set_ua`]
    ///
    /// [`HttpClient::set_ua`]: crate::github::HttpClient::set_ua
    pub fn user_agent(self, user_agent: String) -> Self {
        let user_agent = Some(user_agent);

        match self {
            ClientBuilder::Unconfigured => ClientBuilder::Configured {
                handler: None,
                auth: None,
                user_agent,
            },
            ClientBuilder::Configured { handler, auth, .. } => ClientBuilder::Configured {
                handler,
                auth,
                user_agent,
            },
        }
    }

    /// Adds an [`Authorization`] instance to the current builder using input
    /// from a file.
    pub fn credentials_file(self, file: &str) -> Self {
        let f = File::open(file).unwrap_or_else(|e| panic!("{}", e));
        let mut buf_reader = BufReader::new(f);
        let mut contents = "".to_owned();

        buf_reader
            .read_to_string(&mut contents)
            .expect("ClientBuilder: Reading file");

        let auth: Option<Authorization> = Some(
            toml::from_str::<OctocatConfig>(contents.as_str())
                .expect("ClientBuilder: Parsing config file")
                .to_personal_auth(),
        );

        match self {
            ClientBuilder::Unconfigured => ClientBuilder::Configured {
                handler: None,
                auth,
                user_agent: None,
            },
            ClientBuilder::Configured {
                handler, user_agent, ..
            } => ClientBuilder::Configured {
                handler,
                auth,
                user_agent,
            },
        }
    }

    /// Adds an [`Authorization`] instance to the current builder using input
    /// from an environment variable.
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
            ClientBuilder::Unconfigured => ClientBuilder::Configured {
                handler: None,
                auth,
                user_agent: None,
            },
            ClientBuilder::Configured {
                handler, user_agent, ..
            } => ClientBuilder::Configured {
                handler,
                auth,
                user_agent,
            },
        }
    }

    /// Adds an [`Authorization`] instance to the current builder.
    pub fn personal_auth(self, username: &str, token: &str) -> Self {
        let auth = Some(Authorization::PersonalToken {
            username: username.to_owned(),
            token: token.to_owned(),
        });

        match self {
            ClientBuilder::Unconfigured => ClientBuilder::Configured {
                handler: None,
                auth,
                user_agent: None,
            },
            ClientBuilder::Configured {
                handler, user_agent, ..
            } => ClientBuilder::Configured {
                handler,
                auth,
                user_agent,
            },
        }
    }

    /// Builds the current builder. In other words, this turns a
    /// [`ClientBuilder`] into a [`Client`]. Requires a handler to be set.
    pub fn build(self) -> Result<Client<T>> {
        match self {
            ClientBuilder::Unconfigured => Err(Error::from(BuildError::NotConfigured)),
            ClientBuilder::Configured {
                handler,
                auth,
                user_agent,
            } => {
                let handler = handler.unwrap();
                Ok(Client::<T>::new(handler, auth, user_agent))
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
