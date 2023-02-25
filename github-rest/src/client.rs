//! This module contains a pre-built client you can use instead of making your
//! own client. you can still do this though by implementing the [`Requester`]
//! trait

use async_trait::async_trait;
use base64::write::EncoderWriter as Base64Encoder;
use github_api_octocat::end_points::{EndPoints, Methods};
use reqwest::header;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Display, io::Write};

use crate::{GithubRestError, Requester};

/// A default implementation of the [`Requester`] trait.
pub struct DefaultRequester {
    client: reqwest::Client,
}

impl DefaultRequester {
    pub fn new<T>(auth: T) -> Self
    where
        T: Display,
    {
        let mut auth_header = b"Basic ".to_vec();

        {
            let mut encoder = Base64Encoder::new(&mut auth_header, base64::STANDARD);

            write!(encoder, "{}", auth).unwrap();
        }

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str("tricked.pro/v2").unwrap(),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            header::HeaderValue::from_str("2022-11-28").unwrap(),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(std::str::from_utf8(&auth_header).expect("Failed to parse authorization!"))
                .unwrap(),
        );

        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();

        DefaultRequester { client }
    }

    pub fn new_none() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str("tricked.pro/v2").unwrap(),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
        );

        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
        DefaultRequester { client }
    }
}

#[async_trait]
impl Requester for DefaultRequester {
    async fn raw_req<T, V>(&self, url: EndPoints, query: Option<&T>, body: Option<V>) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Self::Body> + std::marker::Send,
    {
        let path = format!("https://api.github.com{}", url.path());

        let mut req = match url.method() {
            Methods::Get => self.client.get(path),
            Methods::Post => self.client.post(path),
            Methods::Put => self.client.put(path),
            Methods::Patch => self.client.patch(path),
            Methods::Delete => self.client.delete(path),
        };

        if let Some(query) = query {
            req = req.query(query)
        }

        if let Some(body) = body {
            req = req.body(body)
        }

        let res = req.send().await?;

        match res.status().as_u16() {
            200..=299 => {}
            401 => {
                return Err(GithubRestError::NotAuthorized(res.text().await?));
            }
            _ => {
                return Err(GithubRestError::ResponseError(res.status(), res.text().await?));
            }
        }
        let txt = res.text().await?;

        Ok(txt)
    }

    async fn req<T, V, A: DeserializeOwned>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<A, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Self::Body> + std::marker::Send,
    {
        let r = self.raw_req(url, query, body).await?;
        Ok(serde_json::from_str(&r)?)
    }
}
