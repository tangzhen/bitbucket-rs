use crate::auth::Authorization;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

#[async_trait]
pub trait RestClient {
    fn uri(&self) -> &str;

    async fn get<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned;

    async fn post<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned;
}

#[derive(Debug, Builder)]
pub struct BitbucketClient {
    http_client: Client,
    uri: String,
    auth: Option<Authorization>,
}

#[inline]
fn format_host_uri(host: &str) -> String {
    format!("https://{}/rest/api/1.0", host)
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
    pub fn with_auth(host: &str, auth: Authorization) -> Self {
        Self {
            uri: format_host_uri(host),
            auth: Some(auth),
            ..Default::default()
        }
    }
}

impl BitbucketClient {
    pub fn new(host: &str) -> Self {
        Self {
            uri: format_host_uri(host),
            ..Default::default()
        }
    }

    pub fn uri(&self) -> &str {
        &self.uri
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
}

#[async_trait]
impl RestClient for BitbucketClient {
    fn uri(&self) -> &str {
        &self.uri
    }

    async fn get<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut builder = self.http_client.get(uri);
        builder = self.maybe_add_auth(builder);
        let res = builder.send().await?.json().await?;
        Ok(res)
    }

    async fn post<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut builder = self.http_client.post(uri);
        builder = self.maybe_add_auth(builder);
        let res = builder.send().await?.json().await?;
        Ok(res)
    }
}

mod tests {
    use super::*;

    #[test]
    fn get_uri_should_work() {
        let client = BitbucketClient::new("stash.test.com");
        assert_eq!("https://stash.test.com/rest/api/1.0", client.uri());
    }

    #[test]
    fn create_with_auth_should_work() {
        let client =
            BitbucketClient::with_auth("stash.test.com", Authorization::new("george", "test"));

        assert!(client.auth().is_some());
        assert_eq!(
            *client.auth().as_ref().unwrap(),
            Authorization::new("george", "test")
        )
    }
}
