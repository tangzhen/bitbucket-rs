use anyhow::Result;
use async_trait::async_trait;
use reqwest::Response;
use serde::{
    Serialize,
    de::DeserializeOwned,
};

pub trait Payload: Serialize + Send + Sync {}

impl<T> Payload for T where T: Serialize + Send + Sync {}

#[async_trait]
pub trait AsyncRestClient {
    fn uri(&self) -> &str;

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
