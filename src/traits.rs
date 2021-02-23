use anyhow::Result;
use async_trait::async_trait;
use reqwest::Response;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait AsyncRestClient {
    fn uri(&self) -> &str;

    async fn get_as<T>(&self, uri: &str) -> Result<T>
        where
            T: DeserializeOwned;

    async fn get(&self, uri: &str) -> Result<Response>;

    async fn post<T>(&self, uri: &str) -> Result<T>
        where
            T: DeserializeOwned;
}
