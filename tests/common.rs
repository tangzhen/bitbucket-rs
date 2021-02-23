use bitbucket_rs::client::{BitbucketClient, Scheme};
use httpmock::MockServer;

pub type Result = anyhow::Result<()>;

pub const CONTENT_TYPE: &'static str = "Content-Type";
pub const CONTENT_TYPE_JSON: &'static str = "application/json; charset=UTF-8";
pub const REST_PATH_PREFIX: &'static str = "/rest/api/1.0";

pub fn make_client(server: &MockServer) -> BitbucketClient {
    BitbucketClient::new(&server.address().to_string(), Scheme::HTTP)
}

pub fn format_path(path: &str) -> String {
    format!("{}/{}", REST_PATH_PREFIX, path)
}