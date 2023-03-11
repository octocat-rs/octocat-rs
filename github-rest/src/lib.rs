#![feature(associated_type_defaults)]
#![deny(rust_2018_idioms)]

use core::fmt;
use thiserror::Error;

use async_trait::async_trait;
pub use github_api_octocat::end_points;
use github_api_octocat::end_points::EndPoints;

#[cfg(not(target_family = "wasm"))]
use reqwest::{Body, StatusCode};

use serde::{de::DeserializeOwned, Serialize};
#[cfg(target_family = "wasm")]
use std::num::NonZeroU16;
#[cfg(target_family = "wasm")]
use worker::wasm_bindgen::JsValue;

#[cfg(feature = "builders")]
pub mod builders;
#[cfg(feature = "client")]
pub mod client;
pub mod methods;
pub mod model;

#[derive(Error, Debug)]
pub enum GithubRestError {
    #[cfg(not(target_family = "wasm"))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(target_family = "wasm")]
    #[error(transparent)]
    WorkerError(#[from] worker::Error),
    #[cfg(target_family = "wasm")]
    ResponseError(NonZeroU16, String),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(not(target_family = "wasm"))]
    ResponseError(StatusCode, String),
    NotAuthorized(String),
    AnyError(),
}

impl fmt::Display for GithubRestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error occurred as you can see")
    }
}

#[async_trait]
pub trait Requester: Send + Sync {
    #[cfg(not(target_family = "wasm"))]
    type Body: From<String> = Body;

    #[cfg(target_family = "wasm")]
    type Body: From<String> = JsValue;

    async fn raw_req<T, V>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Self::Body> + std::marker::Send;

    async fn req<T, V, A: DeserializeOwned>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<A, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Self::Body> + std::marker::Send;
}
