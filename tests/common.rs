#![allow(dead_code)]

use bitbucket_rs::client::BitbucketClient;
use bitbucket_rs::Scheme;
use httpmock::MockServer;
use bitbucket_rs::auth::Authorization;

pub type Result = anyhow::Result<()>;

pub const CONTENT_TYPE: &'static str = "Content-Type";
pub const CONTENT_TYPE_JSON: &'static str = "application/json; charset=UTF-8";
pub const REST_PATH_PREFIX: &'static str = "/rest/api/1.0";

pub fn make_client(server: &MockServer) -> BitbucketClient {
    BitbucketClient::with_auth(&server.address().to_string(), Scheme::HTTP, Authorization::Bear("token".to_owned()))
}

pub fn format_path(path: &str) -> String {
    format!("{}/{}", REST_PATH_PREFIX, path)
}

pub struct TestContext<'c, R> {
    server: &'c MockServer,
    client: &'c BitbucketClient,
    resource: R,
}

impl<'c, R> TestContext<'c, R> {
    pub fn new(server: &'c MockServer, client: &'c BitbucketClient, resource: R) -> Self {
        Self {
            server,
            client,
            resource,
        }
    }

    pub fn server(&self) -> &MockServer {
        self.server
    }

    pub fn client(&self) -> &BitbucketClient {
        self.client
    }

    pub fn resource(&self) -> &R {
        &self.resource
    }
}

#[cfg(test)]
#[macro_use]
mod tests {
    macro_rules! __context {
        ($server:ident, $client:ident) => {
            let $server = std::boxed::Box::new(MockServer::start_async().await);
            let $server = std::boxed::Box::leak($server);
            let $client = std::boxed::Box::new(crate::common::make_client(&$server));
            let $client = std::boxed::Box::leak($client);
        };
    }

    #[macro_export]
    macro_rules! context {
        ($resource_type:tt) => {{
            __context!(server, client);
            let resource = <bitbucket_rs::resources::$resource_type<_>>::new(client);
            crate::common::TestContext::new(server, client, resource)
        }};

        ($resource_type:tt, $($project:expr)*) => {{
            __context!(server, client);
            let resource = <bitbucket_rs::resources::$resource_type<_>>::new(client, $($project)*);
            crate::common::TestContext::new(server, client, resource)
        }};

        ($resource_type:tt, $($project:expr)*, $($repo:expr)*) => {{
            __context!(server, client);
            let resource = <bitbucket_rs::resources::$resource_type<_>>::new(client, $($project)*, $($repo)*);
            crate::common::TestContext::new(server, client, resource)
        }};
    }
}
