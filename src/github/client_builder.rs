use crate::github::{handler::EventHandler, util::Authorization, Client};

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

    /// Adds an [`Authorization`] instance to the current builder using input
    /// from an environment variable.
    #[allow(unused_variables)]
    pub fn credentials_env_var(self, username_var: &str, token_var: &str) -> Self {
        todo!("This logic is a little annoying so I won't write it just yet");
        todo!("Expand this method once other auth methods are supported")
    }

    /// Adds an [`Authorization`] instance to the current builder.
    pub fn personal_auth(self, username: String, token: String) -> Self {
        let auth = Some(Authorization::PersonalToken { username, token });
        match self {
            ClientBuilder::Unconfigured => ClientBuilder::Configured { handler: None, auth },
            ClientBuilder::Configured { handler, .. } => ClientBuilder::Configured { handler, auth },
        }
    }

    /// Builds the current builder. In other words, this turns a
    /// [`ClientBuilder`] into a [`Client`]
    pub fn build(self) -> Client<T> {
        todo!("Actually let the builder build")
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
