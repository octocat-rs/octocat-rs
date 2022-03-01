#![feature(associated_type_defaults)]
#![deny(rust_2018_idioms)]

use core::fmt;
use std::error::Error;

use async_trait::async_trait;
pub use github_api::end_points;
use github_api::end_points::EndPoints;

#[cfg(not(target_family = "wasm"))]
use reqwest::{Body, StatusCode};

use serde::{de::DeserializeOwned, Serialize};
#[cfg(target_family = "wasm")]
use worker::wasm_bindgen::JsValue;

#[cfg(feature = "builders")]
pub mod builders;
#[cfg(feature = "client")]
pub mod client;
pub mod methods;
pub mod model;

#[derive(Debug)]
pub enum GithubRestError {
    #[cfg(not(target_family = "wasm"))]
    ReqwestError(reqwest::Error),
    #[cfg(target_family = "wasm")]
    WorkerError(worker::Error),
    #[cfg(target_family = "wasm")]
    ResponseError(u16, String),
    JsonError(serde_json::Error),
    #[cfg(not(target_family = "wasm"))]
    ResponseError(StatusCode, String),
    AnyError(),
}

impl fmt::Display for GithubRestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error ocurred as you can see")
    }
}

impl Error for GithubRestError {}

#[cfg(not(target_family = "wasm"))]
impl From<reqwest::Error> for GithubRestError {
    fn from(e: reqwest::Error) -> Self {
        GithubRestError::ReqwestError(e)
    }
}

#[cfg(target_family = "wasm")]
impl From<worker::Error> for GithubRestError {
    fn from(e: worker::Error) -> Self {
        GithubRestError::WorkerError(e)
    }
}

impl From<serde_json::Error> for GithubRestError {
    fn from(e: serde_json::Error) -> Self {
        GithubRestError::JsonError(e)
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
