use crate::{auth::Authorization, models::get::BitbucketErrors, traits::AsyncRestClient};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResult<R> {
    Ok(R),
    Err(BitbucketErrors),
}

impl<R> ApiResult<R> {
    pub fn to_result(self) -> Result<R> {
        match self {
            ApiResult::Ok(r) => Ok(r),
            ApiResult::Err(e) => Err(anyhow::Error::new(e)),
        }
    }
}

pub enum Scheme {
    HTTP,
    HTTPS,
}

#[derive(Debug, Builder)]
pub struct BitbucketClient {
    http_client: Client,
    uri: String,
    auth: Option<Authorization>,
}

#[inline]
fn format_host_uri(host: &str, scheme: Scheme) -> String {
    let scheme = match scheme {
        Scheme::HTTP => "http",
        Scheme::HTTPS => "https",
    };

    format!("{}://{}/rest/api/1.0", scheme, host)
}

impl Default for BitbucketClient {
    fn default() -> Self {
        Self {
            http_client: Client::new(),
            uri: String::new(),
            auth: None,
        }
    }
}

impl BitbucketClient {
    pub fn with_auth(host: &str, scheme: Scheme, auth: Authorization) -> Self {
        Self {
            uri: format_host_uri(host, scheme),
            auth: Some(auth),
            ..Default::default()
        }
    }
}

impl BitbucketClient {
    pub fn new(host: &str, scheme: Scheme) -> Self {
        Self {
            uri: format_host_uri(host, scheme),
            ..Default::default()
        }
    }

    pub fn auth(&self) -> &Option<Authorization> {
        &self.auth
    }

    #[inline]
    fn maybe_add_auth(&self, builder: RequestBuilder) -> RequestBuilder {
        if let Some(ref auth) = self.auth {
            builder.basic_auth(auth.username(), Some(auth.password()))
        } else {
            builder
        }
    }

    async fn perform<F>(&self, method: F) -> Result<Response>
        where
            F: Fn() -> RequestBuilder,
    {
        let mut builder = method();
        builder = self.maybe_add_auth(builder);
        let resp = builder.send().await?;
        Ok(resp)
    }
}

#[async_trait]
impl AsyncRestClient for BitbucketClient {
    fn uri(&self) -> &str {
        &self.uri
    }

    async fn get_as<T>(&self, uri: &str) -> Result<T>
        where
            T: DeserializeOwned,
    {
        self.get(uri)
            .await?
            .json::<ApiResult<T>>()
            .await?
            .to_result()
    }

    async fn get(&self, uri: &str) -> Result<Response> {
        let resp = self.perform(|| self.http_client.get(uri)).await?;
        println!("{:#?}", resp.text().await?);
        self.perform(|| self.http_client.get(uri)).await
    }

    async fn post<T, P>(&self, uri: &str, payload: Option<P>) -> Result<T>
        where
            T: DeserializeOwned,
            P: Serialize + Send + Sync,
    {
        let res = self
            .perform(|| {
                let mut builder = self.http_client.post(uri);
                if let Some(payload) = &payload {
                    builder = builder.json(payload);
                }
                builder
            })
            .await?
            .json()
            .await?;
        Ok(res)
    }
}
