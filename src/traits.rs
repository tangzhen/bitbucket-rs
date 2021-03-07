use crate::Scheme;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Response;
use serde::{de::DeserializeOwned, Serialize};

pub trait Payload: Serialize + Send + Sync {}

impl<T> Payload for T where T: Serialize + Send + Sync {}

#[async_trait]
pub trait AsyncRestClient {
    fn host(&self) -> &str;

    fn scheme(&self) -> &Scheme;

    async fn get(&self, uri: &str) -> Result<Response>;

    async fn get_as<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned;

    async fn post<T, P>(&self, uri: &str, payload: Option<P>) -> Result<T>
    where
        T: DeserializeOwned,
        P: Payload;

    async fn put<T, P>(&self, uri: &str, payload: Option<P>) -> Result<T>
    where
        T: DeserializeOwned,
        P: Payload;

    async fn delete(&self, uri: &str) -> Result<()>;
}

#[async_trait]
impl<C> AsyncRestClient for Box<C>
where
    C: AsyncRestClient + ?Sized + Sync + Send,
{
    fn host(&self) -> &str {
        (**self).host()
    }

    fn scheme(&self) -> &Scheme {
        (**self).scheme()
    }

    async fn get(&self, uri: &str) -> Result<Response> {
        (**self).get(uri).await
    }

    async fn get_as<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        (**self).get_as(uri).await
    }

    async fn post<T, P>(&self, uri: &str, payload: Option<P>) -> Result<T>
    where
        T: DeserializeOwned,
        P: Payload,
    {
        (**self).post(uri, payload).await
    }

    async fn put<T, P>(&self, uri: &str, payload: Option<P>) -> Result<T>
    where
        T: DeserializeOwned,
        P: Payload,
    {
        (**self).put(uri, payload).await
    }

    async fn delete(&self, uri: &str) -> Result<()> {
        (**self).delete(uri).await
    }
}
