use async_trait::async_trait;
use github_rest::{
    methods::prelude::{EndPoints, Methods},
    GithubRestError,
};
use reqwest::{
    header,
    header::{HeaderMap, HeaderValue},
    Body, RequestBuilder,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::time::Duration;

use crate::github::Authorization;

/// An implementor of the [`Requester`] trait.
///
/// [`Requester`]: github_rest::Requester
pub struct HttpClient {
    client: reqwest::Client,
    auth: Option<Authorization>,
}

impl HttpClient {
    // TODO: Allow setting custom UA (will require refactoring outside of this
    // module)
    pub fn new(auth: Option<Authorization>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(header::USER_AGENT, HeaderValue::from_str("Octocat-rs").unwrap());
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
        );

        Self {
            client: reqwest::ClientBuilder::new()
                .default_headers(headers)
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
            auth,
        }
    }

    pub fn set_auth(&mut self, auth: Authorization) {
        self.auth = Some(auth);
    }

    fn auth_headers(&self, req: RequestBuilder) -> RequestBuilder {
        if let Some(auth) = &self.auth {
            match auth {
                Authorization::PersonalToken { username, token } => req.basic_auth(username, Some(token)),
            }
        } else {
            req
        }
    }
}

#[async_trait]
impl github_rest::Requester for HttpClient {
    /// Returns the API response as a [`String`].
    async fn raw_req<T, V>(&self, url: EndPoints, query: Option<&T>, body: Option<V>) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + Send + Sync,
        V: Into<Body> + Send,
    {
        let path = format!("https://api.github.com{}", url.path());

        let mut req = match url.method() {
            Methods::Get => self.auth_headers(self.client.get(path)),
            Methods::Post => self.auth_headers(self.client.post(path)),
            Methods::Put => self.auth_headers(self.client.put(path)),
            Methods::Patch => self.auth_headers(self.client.patch(path)),
            Methods::Delete => self.auth_headers(self.client.delete(path)),
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
        T: Serialize + ?Sized + Send + Sync,
        V: Into<Body> + Send,
    {
        let r = self.raw_req(url, query, body).await?;
        Ok(serde_json::from_str(&r)?)
    }
}
