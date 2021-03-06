use crate::uri_builders::{UriBuilder, BuildResult};

#[derive(Debug, Clone)]
pub struct DiffUriBuilder<B> {
    builder: B,
}

impl<B> DiffUriBuilder<B> where B: UriBuilder {
    pub fn new(builder: B) -> Self {
        Self { builder }
    }

    pub fn path(self, path: &str) -> WithDiffPathUriBuilder<B> {
        WithDiffPathUriBuilder::new(self, path)
    }
}

impl<B> UriBuilder for DiffUriBuilder<B> where B: UriBuilder {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/diff", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithDiffPathUriBuilder<'r, B> {
    builder: DiffUriBuilder<B>,
    path: &'r str,
}

impl<'r, B> WithDiffPathUriBuilder<'r, B> where B: UriBuilder {
    pub fn new(builder: DiffUriBuilder<B>, path: &'r str) -> Self {
        Self { builder, path }
    }
}

impl<'r, B> UriBuilder for WithDiffPathUriBuilder<'r, B> where B: UriBuilder {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.path);
        Ok(uri)
    }
}
