use crate::{
    auth::Authorization,
    models::get::BitbucketErrors,
    traits::{AsyncRestClient, Payload},
    Scheme,
};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Deserialize};

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

#[derive(Debug, Builder)]
pub struct BitbucketClient {
    http_client: Client,
    host: String,
    scheme: Scheme,
    auth: Option<Authorization>,
}

impl Default for BitbucketClient {
    fn default() -> Self {
        Self {
            http_client: Client::new(),
            host: String::new(),
            scheme: Scheme::HTTP,
            auth: None,
        }
    }
}

impl BitbucketClient {
    pub fn with_auth(host: &str, scheme: Scheme, auth: Authorization) -> Self {
        Self {
            host: host.to_owned(),
            scheme,
            auth: Some(auth),
            ..Default::default()
        }
    }
}

impl BitbucketClient {
    pub fn new(host: &str, scheme: Scheme) -> Self {
        Self {
            host: host.to_owned(),
            scheme,
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

    #[inline]
    fn maybe_add_payload<P>(&self, builder: RequestBuilder, payload: Option<&P>) -> RequestBuilder
    where
        P: Payload,
    {
        if let Some(payload) = payload {
            builder.json(payload)
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

    async fn perform_as<T, F>(&self, method: F) -> Result<T>
    where
        F: Fn() -> RequestBuilder,
        T: DeserializeOwned,
    {
        self.perform(method)
            .await?
            .json::<ApiResult<T>>()
            .await?
            .to_result()
    }
}

#[async_trait]
impl AsyncRestClient for BitbucketClient {
    fn host(&self) -> &str {
        &self.host
    }

    fn scheme(&self) -> &Scheme {
        &self.scheme
    }

    async fn get(&self, uri: &str) -> Result<Response> {
        self.perform(|| self.http_client.get(uri)).await
    }

    async fn get_as<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.perform_as(|| self.http_client.get(uri)).await
    }

    async fn post<T, P>(&self, uri: &str, payload: Option<P>) -> Result<T>
    where
        T: DeserializeOwned,
        P: Payload,
    {
        self.perform_as(|| {
            let builder = self.http_client.post(uri);
            self.maybe_add_payload(builder, payload.as_ref())
        })
        .await
    }

    async fn put<T, P>(&self, uri: &str, payload: Option<P>) -> Result<T>
    where
        T: DeserializeOwned,
        P: Payload,
    {
        self.perform_as(|| {
            let builder = self.http_client.put(uri);
            self.maybe_add_payload(builder, payload.as_ref())
        })
        .await
    }

    async fn delete(&self, uri: &str) -> Result<()> {
        let resp = self.perform(|| self.http_client.delete(uri)).await?;
        let status = resp.status();
        let is_error = status.is_client_error() || status.is_server_error();

        if !is_error {
            Ok(())
        } else {
            let errors: BitbucketErrors = resp.json().await?;
            Err(anyhow::Error::new(errors))
        }
    }
}
