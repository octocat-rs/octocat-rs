//! Contains the [`Command`] interface used in the [`handler`] module by the
//! [`EventHandler`] trait.
//!
//! [`handler`]: crate::handler
//! [`EventHandler`]: crate::handler::EventHandler

use std::fmt::Debug;

use futures::{
    future,
    future::{Future, FutureExt},
};

pub type BoxFuture<T> = future::BoxFuture<'static, T>;

/// This struct contains code adapted from the [`Command`](https://github.com/iced-rs/iced/blob/0.3/futures/src/command.rs) struct written in the [iced](https://github.com/iced-rs/iced) library. It is currently identical to the aforementioned struct, however modifications will be made to fit our needs with time.
///
/// Kudos to Hecrj for his awesome system- I don't know where I'd be without it
pub struct Command<T>
where
    T: std::fmt::Debug + Send,
{
    to_be_performed: Vec<BoxFuture<T>>,
}

impl<T> Command<T>
where
    T: std::fmt::Debug + Send,
{
    /// Creates an empty [`Command`] that does not contain any futures.
    pub fn none() -> Self {
        Self {
            to_be_performed: Vec::new(),
        }
    }

    /// Creates a [`Command`] that executes the given future when run.
    pub fn perform<A: Debug + Send>(
        future: impl Future<Output = T> + Send + 'static,
        f: impl Fn(T) -> A + Send + 'static,
    ) -> Command<A> {
        Command {
            to_be_performed: vec![Box::pin(future.map(f))],
        }
    }

    /// Creates a [`Command`] that executes each future at once when run.
    pub fn perform_multiple(futures: impl IntoIterator<Item = Command<T>>) -> Self {
        Self {
            to_be_performed: futures
                .into_iter()
                .flat_map(|command| command.to_be_performed)
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.to_be_performed.len() == 0
    }

    /// Transforms the result of a [`Command`].
    #[allow(clippy::redundant_closure)] // Known bug with Clippy
    pub fn map<A: Debug + Send>(mut self, f: impl Fn(T) -> A + 'static + Send + Sync) -> Command<A>
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

                    Box::pin(future.map(move |result| f(result))) as BoxFuture<A>
                })
                .collect(),
        }
    }

    /// Drops the current [`Command`] instance and returns its futures.
    pub fn into_futures(self) -> Vec<BoxFuture<T>> {
        self.to_be_performed
    }
}

impl<T, A> From<A> for Command<T>
where
    A: Future<Output = T> + Send + 'static,
    T: Debug + Send,
{
    fn from(future: A) -> Self {
        Self {
            to_be_performed: vec![future.boxed()],
        }
    }
}
