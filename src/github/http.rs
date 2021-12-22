use crate::github::Authorization;
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

/// An implementor of the [`Requester`] trait.
///
/// [`Requester`]: github_rest::Requester
pub struct HttpClient {
    client: reqwest::Client,
    auth: Authorization,
}

impl HttpClient {
    pub fn new(auth: Authorization) -> Self {
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

    fn http_client(&self, req: RequestBuilder) -> RequestBuilder {
        match &self.auth {
            Authorization::PersonalToken { username, token } => req.basic_auth(username, Some(token)),
        }
    }
}

#[async_trait]
impl github_rest::Requester for HttpClient {
    async fn raw_req<T, V>(&self, url: EndPoints, query: Option<&T>, body: Option<V>) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + Send + Sync,
        V: Into<Body> + Send,
    {
        let path = format!("https://api.github.com{}", url.path());

        let mut req = match url.method() {
            Methods::Get => self.http_client(self.client.get(path)),
            Methods::Post => self.http_client(self.client.post(path)),
            Methods::Put => self.http_client(self.client.put(path)),
            Methods::Patch => self.http_client(self.client.patch(path)),
            Methods::Delete => self.http_client(self.client.delete(path)),
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
