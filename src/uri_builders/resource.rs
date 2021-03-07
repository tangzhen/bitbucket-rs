use crate::uri_builders::{
    AdminUriBuilder, BuildResult, ProjectUriBuilder, UriBuilder, REST_API_URI,
};
use crate::Scheme;

#[derive(Debug, Clone)]
pub struct ResourceUriBuilder<'r> {
    scheme: Scheme,
    host: Option<&'r str>,
}

impl<'r> Default for ResourceUriBuilder<'r> {
    fn default() -> Self {
        Self {
            scheme: Scheme::HTTP,
            host: None,
        }
    }
}

impl<'r> ResourceUriBuilder<'r> {
    pub fn scheme(mut self, scheme: &Scheme) -> Self {
        self.scheme = scheme.clone();
        self
    }

    pub fn host(mut self, host: &'r str) -> Self {
        self.host = Some(host);
        self
    }

    pub fn admin(self) -> AdminUriBuilder<'r> {
        AdminUriBuilder::new(self)
    }

    pub fn projects(self) -> ProjectUriBuilder<'r> {
        ProjectUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for ResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let host = self.host.ok_or("host must be initialized")?;
        let scheme = match &self.scheme {
            Scheme::HTTP => "http",
            Scheme::HTTPS => "https",
        };

        Ok(format!("{}://{}/{}", scheme, host, REST_API_URI))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::{base_uri, TEST_HOST};

    #[test]
    fn resource_uri_builder_requires_host() {
        let uri = ResourceUriBuilder::default().build();
        assert!(uri.is_err());
        assert_eq!(uri.unwrap_err().msg(), "host must be initialized");
    }

    #[test]
    fn resource_uri_builder_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).build();
        assert_uri!(uri, base_uri());
    }

    #[test]
    fn resource_uri_with_scheme_works() {
        let uri = ResourceUriBuilder::default()
            .scheme(&Scheme::HTTPS)
            .host(TEST_HOST)
            .build();
        assert_uri!(uri, format!("https://{}/{}", TEST_HOST, REST_API_URI));
    }

    #[test]
    fn resource_clone_works() {
        let builder = ResourceUriBuilder::default().host(TEST_HOST);
        let uri = builder.clone().host("stash.clone.com").build();
        assert_uri!(uri, "http://stash.clone.com/rest/api/1.0");
        let uri = builder.build();
        assert_uri!(uri, base_uri());
    }
}
