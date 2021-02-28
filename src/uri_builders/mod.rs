const REST_API_URI: &'static str = "rest/api/1.0";

type BuildResult = Result<String, String>;

pub trait UriBuilder {
    fn build(&self) -> BuildResult;
}

#[derive(Debug)]
pub struct TerminalUriBuilder<B> {
    builder: B
}

impl<B> TerminalUriBuilder<B>
    where
        B: UriBuilder
{
    pub fn new(builder: B) -> Self {
        Self { builder }
    }

    pub fn build(&self) -> BuildResult {
        self.builder.build()
    }
}

mod resource;
mod admin;
mod project;
mod repository;
mod branch;
mod commit;

pub use resource::*;
pub use admin::*;
pub use project::*;
pub use repository::*;
pub use branch::*;
pub use commit::*;

#[cfg(test)]
mod tests {
    use crate::uri_builders::REST_API_URI;

    pub const TEST_HOST: &'static str = "stash.test.com";
    pub const TEST_PROJECT: &'static str = "RRJ";
    pub const TEST_REPO: &'static str = "REPO";

    pub fn base_uri() -> String {
        format!("http://{}/{}", TEST_HOST, REST_API_URI)
    }
}