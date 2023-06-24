//! Contains [`HttpClient`].

#[cfg(all(target_family = "wasm", feature = "workers"))]
use std::{io::Write, num::NonZeroU16};

use async_trait::async_trait;
#[cfg(all(target_family = "wasm", feature = "workers"))]
use base64::write::EncoderWriter as Base64Encoder;
#[cfg(feature = "native")]
use reqwest::{
    header,
    header::{HeaderMap, HeaderValue},
    RequestBuilder,
};
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "native")]
use tokio::time::Duration;
#[cfg(all(target_family = "wasm", feature = "workers"))]
use worker::{Fetch, Headers, Method, Request, RequestInit};

use github_rest::{
    methods::prelude::{EndPoints, Methods},
    GithubRestError,
};

use crate::github::Authorization;

const USER_AGENT_PARSE_ERROR: &str = "HttpClient: Parsing user agent";
const ACCEPT_HEADER_PARSE_ERROR: &str = "HttpClient: Parsing accept header";

/// An implementer of the [`Requester`] trait. This is all most users will need,
/// however it may be helpful to look at the trait implementation details here
/// if you're writing your own implementation.
///
/// [`Requester`]: github_rest::Requester
pub struct HttpClient {
    #[cfg(feature = "native")]
    client: reqwest::Client,
    #[cfg(all(target_family = "wasm", feature = "workers"))]
    user_agent: Option<String>,
    auth: Option<Authorization>,
}

impl HttpClient {
    /// Creates a new `HttpClient`.
    #[cfg(feature = "native")]
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

    /// Creates a new `HttpClient` with no authorization.
    pub fn new_none() -> Self {
        Self::new(None, None)
    }

    /// Creates a new `HttpClient`.
    #[cfg(all(target_family = "wasm", feature = "workers"))]
    pub fn new(auth: Option<Authorization>, user_agent: Option<String>) -> Self {
        HttpClient { user_agent, auth }
    }

    /// Updates the authorization used by the current client.
    pub fn set_auth(&mut self, auth: Authorization) {
        self.auth = Some(auth);
    }

    /// Set the user agent used by the current client.
    #[cfg(feature = "native")]
    pub fn set_ua(&mut self, user_agent: &str) {
        let mut headers = HeaderMap::new();

        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_str(user_agent).expect(USER_AGENT_PARSE_ERROR),
        );
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_str("application/vnd.github.v3+json").expect(ACCEPT_HEADER_PARSE_ERROR),
        );

        self.client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
    }

    /// Set the user agent used by the current client.
    #[cfg(all(target_family = "wasm", feature = "workers"))]
    pub fn set_ua(&mut self, user_agent: String) {
        self.user_agent = Some(user_agent);
    }

    #[cfg(feature = "native")]
    fn http_auth(&self, req: RequestBuilder) -> RequestBuilder {
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
    #[cfg(feature = "native")]
    async fn raw_req<T, V>(&self, url: EndPoints, query: Option<&T>, body: Option<V>) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + Send + Sync,
        V: Into<Self::Body> + Send,
    {
        let req = {
            let path = format!("https://api.github.com{}", url.path());

            let mut req = self.http_auth(match url.method() {
                Methods::Get => self.client.get(path),
                Methods::Post => self.client.post(path),
                Methods::Put => self.client.put(path),
                Methods::Patch => self.client.patch(path),
                Methods::Delete => self.client.delete(path),
            });

            if let Some(query) = query {
                req = req.query(query);
            }

            if let Some(body) = body {
                req = req.body(body);
            }
            
            req
        };

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

    /// Returns the API response as a [`String`].
    #[cfg(all(target_family = "wasm", feature = "workers"))]
    async fn raw_req<T, V>(&self, url: EndPoints, query: Option<&T>, body: Option<V>) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + Send + Sync,
        V: Into<Self::Body> + Send,
    {
        let mut path = format!("https://api.github.com{}", url.path());

        if let Some(q) = query {
            path.push_str(serde_urlencoded::to_string(q).expect("Invalid query").as_str());
        }

        futures::executor::block_on(async move {
            let headers = {
                let mut headers = Headers::new();

                headers
                    .append("accept", "application/vnd.github.v3+json")
                    .expect(ACCEPT_HEADER_PARSE_ERROR);

                if let Some(auth) = &self.auth {
                    match auth {
                        Authorization::PersonalToken { username, token } => {
                            let mut header_value = b"Basic ".to_vec();

                            {
                                let mut encoder =
                                    Base64Encoder::new(&mut header_value, &base64::engine::general_purpose::STANDARD);

                                write!(encoder, "{username}:").unwrap();
                                write!(encoder, "{token}").unwrap();
                            }

                            headers
                                .append(
                                    "authorization",
                                    std::str::from_utf8(&header_value).expect("Failed to parse header value"),
                                    )
                                .unwrap();
                        }
                    }
                }

                if let Some(ua) = &self.user_agent {
                    headers.append("user-agent", ua).expect(USER_AGENT_PARSE_ERROR);
                }

                headers
            };

            let init = {
                let mut init = RequestInit::new();
                // I don't want to know. To future generations: I am sorry. 
                init.with_method(BadWrapper::new(url.method()).into());

                init.with_headers(headers);

                if let Some(body) = body {
                    init.with_body(Some(body.into()));
                }

                init
            };

            let req = Request::new_with_init(path.as_str(), &init)?;

            let mut res = Fetch::Request(req).send().await?;

            match res.status_code() {
                200..=299 => {}
                401 => {
                    return Err(GithubRestError::NotAuthorized(res.text().await?));
                }
                _ => {
                    return Err(GithubRestError::ResponseError(
                        NonZeroU16::new(res.status_code()).unwrap(),
                        res.text().await?,
                    ));
                }
            }

            Ok(res.text().await?)
        })
    }

    /// A function for performing HTTP requests utilizing the [`EndPoints`]
    /// enum.
    ///
    /// Usage example:
    ///
    /// ```rust
    /// # use github_rest::{methods::GetCommitsBody, model::commits::Commits, Requester};
    /// # use github_api_octocat::end_points::EndPoints;
    /// # use octocat_rs::HttpClient;
    /// #
    /// HttpClient::new_none()
    ///     .req::<GetCommitsBody, String, Commits>(
    ///         EndPoints::GetReposownerrepoCommits("octocat-rs".to_owned(), "octocat-rs".to_owned()),
    ///             None,
    ///             None,
    ///         )
    ///         .await;
    /// ```
    ///
    ///
    /// [`EndPoints`]: github_api_octocat::end_points::EndPoints
    async fn req<T, V, A: DeserializeOwned>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<A, GithubRestError>
    where
        T: Serialize + ?Sized + Send + Sync,
        V: Into<Self::Body> + Send,
    {
        let r = self.raw_req(url, query, body).await?;
        Ok(serde_json::from_str(&r)?)
    }
}

/// I would like to apologize to the world for this crime against nature. 
#[cfg(all(target_family = "wasm", feature = "workers"))]
struct BadWrapper<T> {
    pub(crate) inner: T,
}

#[cfg(all(target_family = "wasm", feature = "workers"))]
impl<T> BadWrapper<T> {
    fn new(inner: T) -> Self {
        BadWrapper { inner }
    }
}

#[cfg(all(target_family = "wasm", feature = "workers"))]
impl From<BadWrapper<Methods>> for Method {
    fn from(v: BadWrapper<Methods>) -> Self {
        match v.inner {
            Methods::Get => Method::Get,
            Methods::Post => Method::Post,
            Methods::Patch => Method::Patch,
            Methods::Delete => Method::Delete,
            Methods::Put => Method::Put,
        }
    }
}
