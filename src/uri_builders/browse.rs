use crate::uri_builders::path::PathUriBuilder;
use crate::uri_builders::{BuildResult, TerminalUriBuilder, UriBuilder};

#[derive(Debug, Clone)]
pub struct BrowseUriBuilder<B> {
    builder: PathUriBuilder<'static, B>,
}

impl<B> BrowseUriBuilder<B>
where
    B: UriBuilder,
{
    pub fn new(builder: B) -> Self {
        let builder = PathUriBuilder::new(builder, "browse");
        Self { builder }
    }

    pub fn path(self, path: &str) -> TerminalUriBuilder<PathUriBuilder<'_, B>> {
        self.builder.path(path)
    }
}

impl<B> UriBuilder for BrowseUriBuilder<B>
where
    B: UriBuilder,
{
    fn build(&self) -> BuildResult {
        self.builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::EmptyUriBuilder;

    fn builder() -> BrowseUriBuilder<EmptyUriBuilder> {
        BrowseUriBuilder::new(EmptyUriBuilder)
    }

    #[test]
    fn browse_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, "/browse");
    }

    #[test]
    fn browse_path_uri_works() {
        let uri = builder().path("home/test").build();
        assert_uri!(uri, "/browse/home/test");
    }
}
