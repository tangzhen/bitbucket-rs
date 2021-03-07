use crate::uri_builders::path::PathUriBuilder;
use crate::uri_builders::{UriBuilder, TerminalUriBuilder, BuildResult};

#[derive(Debug, Clone)]
pub struct FileUriBuilder<B> {
    builder: PathUriBuilder<'static, B>
}

impl<B> FileUriBuilder<B> where B: UriBuilder {
    pub fn new(builder: B) -> Self {
        let builder = PathUriBuilder::new(builder, "files");
        Self { builder }
    }

    pub fn path(self, path: &str) -> TerminalUriBuilder<PathUriBuilder<'_, B>> {
        self.builder.path(path)
    }
}

impl<B> UriBuilder for FileUriBuilder<B> where B: UriBuilder {
    fn build(&self) -> BuildResult {
        self.builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::EmptyUriBuilder;

    fn builder() -> FileUriBuilder<EmptyUriBuilder> {
        FileUriBuilder::new(EmptyUriBuilder)
    }

    #[test]
    fn file_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, "/files");
    }

    #[test]
    fn file_path_uri_works() {
        let uri = builder().path("home/test").build();
        assert_uri!(uri, "/files/home/test");
    }
}