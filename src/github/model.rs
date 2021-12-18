use std::future::Future;

use futures::{future, FutureExt};

type BoxFuture<T> = future::BoxFuture<'static, T>;

#[derive(Debug, Clone)]
pub enum Authentication {
    CredentialsFile(String),
    EnvironmentVariable(String),
}

/// NOTE: To be deprecated.
/// Used to hold the credentials for [`PersonalClient`].
#[derive(Debug, Clone)]
pub enum Authorization {
    Personal {
        username: String,
        access_token: String,
    },
}

/// This struct contains code adapted from the [`Command`](https://github.com/iced-rs/iced/blob/0.3/futures/src/command.rs) struct written in the [iced](https://github.com/iced-rs/iced) library. It is currently nearly identical to the aforementioned struct, however more modifications will be made to fit our needs with time.
///
/// Kudos to Hecrj for his awesome system- I don't know where I'd be without it
pub struct Command<T> {
    to_be_performed: Vec<BoxFuture<T>>,
}

impl<T> Command<T> {
    pub fn none() -> Self {
        Self {
            to_be_performed: Vec::new(),
        }
    }

    // TODO: Message stuff
    pub fn perform<A>(
        future: impl Future<Output = T> + Send + 'static,
        f: impl Fn(T) -> A + 'static + Send,
    ) -> Command<A> {
        Command {
            to_be_performed: vec![Box::pin(future.map(f))],
        }
    }

    #[allow(clippy::redundant_closure)]
    pub fn map<A>(
        mut self,
        f: impl Fn(T) -> A + 'static + Send + Sync,
    ) -> Command<A>
    where
        T: 'static,
    {
        let f = std::sync::Arc::new(f);

        Command {
            to_be_performed: self
                .to_be_performed
                .drain(..)
                .map(|future| {
                    let f = f.clone();

                    Box::pin(future.map(move |result| f(result)))
                        as BoxFuture<A>
                })
                .collect(),
        }
    }

    pub fn perform_many(
        commands: impl IntoIterator<Item = Command<T>>,
    ) -> Self {
        Self {
            to_be_performed: commands
                .into_iter()
                .flat_map(|command| command.to_be_performed)
                .collect(),
        }
    }
}
