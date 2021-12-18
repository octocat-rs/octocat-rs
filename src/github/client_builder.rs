use crate::github::traits::Handler;

type EventHandler<T> = Box<dyn Handler<Message = T>>;

#[derive(Default)]
pub struct PersonalClientBuilder<T> {
    config: Config<T>,
}

pub enum Config<T> {
    Unconfigured,
    Configured {
        handler: Option<EventHandler<T>>,
        credentials: Option<Credentials>,
    },
}

pub enum Credentials {
    File(String),
}

impl<T> PersonalClientBuilder<T> {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn event_handler(self, event_handler: EventHandler<T>) -> Self {
        let handler = Some(event_handler);
        let config = match self.config {
            Config::Unconfigured => Config::Configured {
                handler,
                credentials: None,
            },
            Config::Configured { credentials, .. } => Config::Configured { handler, credentials },
        };

        Self { config }
    }

    pub fn credentials_file(self, file: &str) -> Self {
        let credentials = Some(Credentials::File(file.to_owned()));

        let config = match self.config {
            Config::Unconfigured => Config::Configured {
                handler: None,
                credentials,
            },
            Config::Configured { handler, .. } => Config::Configured { handler, credentials },
        };

        Self { config }
    }
}

impl<T> Default for Config<T> {
    fn default() -> Self {
        Self::Unconfigured
    }
}
