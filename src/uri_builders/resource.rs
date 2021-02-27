use crate::uri_builders::{UriBuilder, BuildResult, REST_API_URI, AdminResourceUriBuilder, ProjectResourceUriBuilder};

#[derive(Debug, Default)]
pub struct ResourceUriBuilder<'r> {
    host: Option<&'r str>,
}

impl<'r> ResourceUriBuilder<'r> {
    pub fn host(mut self, host: &'r str) -> Self {
        self.host = Some(host);
        self
    }

    pub fn admin(self) -> AdminResourceUriBuilder<'r> {
        AdminResourceUriBuilder::new(self)
    }

    pub fn projects(self) -> ProjectResourceUriBuilder<'r> {
        ProjectResourceUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for ResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let host = self.host.ok_or("host must be initialized")?;
        Ok(format!("http://{}/{}", host, REST_API_URI))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::TEST_HOST;

    #[test]
    fn resource_uri_builder_requires_host() {
        let uri = ResourceUriBuilder::default().build();
        assert!(uri.is_err());
        assert_eq!(uri.unwrap_err(), "host must be initialized");
    }

    #[test]
    fn resource_uri_builder_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0");
    }
}