use crate::uri_builders::{UriBuilder, BuildResult, TerminalUriBuilder};
use crate::uri_builders::path::PathUriBuilder;

#[derive(Debug, Clone)]
pub struct DiffUriBuilder<B> {
    builder: PathUriBuilder<'static, B>,
}

impl<B> DiffUriBuilder<B> where B: UriBuilder {
    pub fn new(builder: B) -> Self {
        let builder = PathUriBuilder::new(builder, "diff");
        Self { builder }
    }

    pub fn path(self, path: &str) -> TerminalUriBuilder<PathUriBuilder<'static, B>> {
        TerminalUriBuilder::new(self.builder, path.to_string())
    }
}

impl<B> UriBuilder for DiffUriBuilder<B> where B: UriBuilder {
    fn build(&self) -> BuildResult {
        self.builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::EmptyUriBuilder;

    fn builder() -> DiffUriBuilder<EmptyUriBuilder> {
        DiffUriBuilder::new(EmptyUriBuilder)
    }

    #[test]
    fn diff_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, "/diff");
    }

    #[test]
    fn diff_path_uri_works() {
        let uri = builder().path("home/test").build();
        assert_uri!(uri, "/diff/home/test");
    }
}