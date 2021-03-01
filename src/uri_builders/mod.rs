use std::fmt;

const REST_API_URI: &'static str = "rest/api/1.0";

#[derive(Debug)]
pub struct BuildError {
    msg: String,
}

impl BuildError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

impl Error for BuildError {}

impl From<String> for BuildError {
    fn from(msg: String) -> Self {
        BuildError::new(msg)
    }
}

impl From<&str> for BuildError {
    fn from(msg: &str) -> Self {
        BuildError::new(msg.to_owned())
    }
}

type BuildResult = Result<String, BuildError>;

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

#[cfg(test)]
#[macro_use]
mod tests {
    use crate::uri_builders::REST_API_URI;

    pub const TEST_HOST: &'static str = "stash.test.com";
    pub const TEST_PROJECT: &'static str = "RRJ";
    pub const TEST_REPO: &'static str = "REPO";

    pub fn base_uri() -> String {
        format!("http://{}/{}", TEST_HOST, REST_API_URI)
    }

    macro_rules! assert_uri {
        ($uri:expr, $expected:expr) => {
            assert!($uri.is_ok());
            assert_eq!($uri.unwrap(), $expected);
        }
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
use std::fmt::Formatter;
use std::error::Error;

