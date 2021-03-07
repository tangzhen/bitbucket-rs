use crate::uri_builders::{UriBuilder, BuildResult, TerminalUriBuilder};

#[derive(Debug, Clone)]
pub struct PathUriBuilder<'r, B> {
    builder: B,
    path: &'r str,
}

impl<'r, B> PathUriBuilder<'r, B> where B: UriBuilder {
    pub fn new(builder: B, path: &'r str) -> Self {
        Self { builder, path }
    }

    pub fn path(self, path: &str) -> TerminalUriBuilder<Self> {
        TerminalUriBuilder::new(self, path.to_string())
    }
}

impl<'r, B> UriBuilder for PathUriBuilder<'r, B> where B: UriBuilder {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.path);
        Ok(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::EmptyUriBuilder;

    fn builder() -> PathUriBuilder<'static, EmptyUriBuilder> {
        PathUriBuilder::new(EmptyUriBuilder, "test")
    }

    #[test]
    fn path_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, "/test");
    }

    #[test]
    fn with_path_uri_works() {
        let uri = builder().path("home/test").build();
        assert_uri!(uri, "/test/home/test");
    }
}
