use std::fmt;

const REST_API_URI: &str = "rest/api/1.0";

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
    builder: B,
    resource: String,
}

impl<'r, B> TerminalUriBuilder<B>
where
    B: UriBuilder,
{
    pub fn new(builder: B, resource: String) -> Self {
        Self { builder, resource }
    }
}

impl<B> UriBuilder for TerminalUriBuilder<B>
where
    B: UriBuilder,
{
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.resource);
        Ok(uri)
    }
}

macro_rules! terminal_resource_fn {
    ($fn_name:ident) => {
        pub fn $fn_name(self) -> crate::uri_builders::TerminalUriBuilder<Self> {
            let function_name = heck::AsKebabCase(std::stringify!($fn_name)).to_string();
            crate::uri_builders::TerminalUriBuilder::new(self, function_name)
        }
    };
}

#[cfg(test)]
#[macro_use]
mod tests {
    use crate::uri_builders::{BuildResult, UriBuilder, REST_API_URI};

    pub const TEST_HOST: &'static str = "stash.test.com";
    pub const TEST_PROJECT: &'static str = "RRJ";
    pub const TEST_REPO: &'static str = "REPO";

    pub fn base_uri() -> String {
        format!("http://{}/{}", TEST_HOST, REST_API_URI)
    }

    pub struct EmptyUriBuilder;

    impl UriBuilder for EmptyUriBuilder {
        fn build(&self) -> BuildResult {
            Ok(String::new())
        }
    }

    macro_rules! assert_uri {
        ($uri:expr, $expected:expr) => {
            assert!($uri.is_ok());
            assert_eq!($uri.unwrap(), $expected);
        };
    }
}

mod admin;
mod branch;
mod browse;
mod commit;
mod diff;
mod file;
mod log;
mod path;
mod permission;
mod project;
mod pull_request;
mod repository;
mod resource;
mod user;

pub use admin::*;
pub use branch::*;
pub use browse::*;
pub use commit::*;
pub use diff::*;
pub use file::*;
pub use log::*;
pub use permission::*;
pub use project::*;
pub use pull_request::*;
pub use repository::*;
pub use resource::*;

use std::error::Error;
use std::fmt::Formatter;
pub use user::*;
