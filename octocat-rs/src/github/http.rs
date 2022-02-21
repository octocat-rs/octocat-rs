use async_trait::async_trait;
#[cfg(not(feature = "workers"))]
use reqwest::{
    header,
    header::{HeaderMap, HeaderValue},
    Body, RequestBuilder,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::time::Duration;

#[cfg(feature = "workers")]
use worker::{Fetch, Request, Method};
use worker::RequestInit;
use worker::ResponseBody::Body;

use github_rest::{
    methods::prelude::{EndPoints, Methods},
    GithubRestError,
};

use crate::github::Authorization;

const USER_AGENT_PARSE_ERROR: &str = "HttpClient: Parsing user agent";
const ACCEPT_HEADER_PARSE_ERROR: &str = "HttpClient: Parsing accept header";

/// An implementer of the [`Requester`] trait.
///
/// [`Requester`]: github_rest::Requester
pub struct HttpClient {
    client: reqwest::Client,
    auth: Option<Authorization>,
}

impl HttpClient {
    pub fn new(auth: Option<Authorization>, user_agent: Option<String>) -> Self {
        let mut headers = HeaderMap::new();

        let user_agent = match user_agent {
            Some(s) => s,
            None => "Octocat-rs".to_owned(),
        };

        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_str(user_agent.as_str()).expect(USER_AGENT_PARSE_ERROR),
        );
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_str("application/vnd.github.v3+json").expect(ACCEPT_HEADER_PARSE_ERROR),
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

    /// Updates the authorization used by the current client.
    pub fn set_auth(&mut self, auth: Authorization) {
        self.auth = Some(auth);
    }

    /// Set the user agent used by the current client.
    pub fn set_ua(&mut self, user_agent: String) {
        let mut headers = HeaderMap::new();

        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_str(user_agent.as_str()).expect(USER_AGENT_PARSE_ERROR),
        );
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_str("application/vnd.github.v3+json").expect(ACCEPT_HEADER_PARSE_ERROR),
        );

        self.client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap()
    }

    #[cfg(not(feature = "workers"))]
    fn http_auth(&self, req: RequestBuilder) -> RequestBuilder {
        if let Some(auth) = &self.auth {
            match auth {
                Authorization::PersonalToken { username, token } => req.basic_auth(username, Some(token)),
            }
        } else {
            req
        }
    }

    #[cfg(feature = "workers")]
    fn http_auth(&self, req: RequestBuilder) -> RequestBuilder {
        todo!()
    }
}

#[async_trait]
impl github_rest::Requester for HttpClient {
    #[cfg(not(feature = "workers"))]
    /// Returns the API response as a [`String`].
    async fn raw_req<T, V>(&self, url: EndPoints, query: Option<&T>, body: Option<V>) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + Send + Sync,
        V: Into<Body> + Send,
    {
        let path = format!("https://api.github.com{}", url.path());

        let mut req = match url.method() {
            Methods::Get => self.http_auth(self.client.get(path)),
            Methods::Post => self.http_auth(self.client.post(path)),
            Methods::Put => self.http_auth(self.client.put(path)),
            Methods::Patch => self.http_auth(self.client.patch(path)),
            Methods::Delete => self.http_auth(self.client.delete(path)),
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

    #[cfg(feature = "workers")]
    /// Returns the API response as a [`String`].
    async fn raw_req<T, V>(&self, url: EndPoints, query: Option<&T>, body: Option<V>) -> Result<String, GithubRestError>
        where
            T: Serialize + ?Sized + Send + Sync,
            V: Into<Body> + Send,
    {
        let path = format!("https://api.github.com{}", url.path());

        let mut req: Request = match url.method() {
            Methods::Get => Request::new(path.as_str(), Method::Get).unwrap(),
            Methods::Post => Request::new(path.as_str(), Method::Post).unwrap(),
            Methods::Patch => Request::new(path.as_str(), Method::Patch).unwrap(),
            Methods::Delete => Request::new(path.as_str(), Method::Delete).unwrap(),
            Methods::Put => Request::new(path.as_str(), Method::Put).unwrap(),
        };

        let _init = RequestInit::new();

        if let Some(query) = query {
            // TODO: Query
        }

        if let Some(body) = body {
            // TODO: Body
        }

        let mut res = Fetch::Request(req).send().await?;

        match res.status_code() {
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
