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

pub use resource::*;
pub use admin::*;
pub use project::*;
pub use repository::*;

#[cfg(test)]
mod tests {
    pub const TEST_HOST: &'static str = "stash.test.com";
}