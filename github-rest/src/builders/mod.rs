use crate::{GithubRestError, Requester};
use async_trait::async_trait;
pub use commits::*;
pub use issues::*;
pub use reactions::*;
use serde::de::DeserializeOwned;

mod commits;
mod issues;
mod reactions;

#[async_trait]
pub trait Builder {
    type Response: DeserializeOwned;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester;
}
