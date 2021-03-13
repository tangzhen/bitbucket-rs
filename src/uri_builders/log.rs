use crate::uri_builders::{BuildResult, ResourceUriBuilder, TerminalUriBuilder, UriBuilder};

#[derive(Debug, Clone)]
pub struct LogUriBuilder<'r> {
    builder: ResourceUriBuilder<'r>,
}

impl<'r> LogUriBuilder<'r> {
    pub fn new(builder: ResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn logger(self, logger: &'r str) -> WithLoggerUriBuilder<'r> {
        WithLoggerUriBuilder::new(self, logger)
    }

    pub fn root_logger(self) -> RootLoggerUriBuilder<'r> {
        RootLoggerUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for LogUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/logs", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithLoggerUriBuilder<'r> {
    builder: LogUriBuilder<'r>,
    logger: &'r str,
}

impl<'r> WithLoggerUriBuilder<'r> {
    pub fn new(builder: LogUriBuilder<'r>, logger: &'r str) -> Self {
        Self { builder, logger }
    }

    pub fn level(self, level: &str) -> TerminalUriBuilder<Self> {
        TerminalUriBuilder::new(self, level.to_string())
    }
}

impl<'r> UriBuilder for WithLoggerUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/logger/{}", self.builder.build()?, self.logger);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct RootLoggerUriBuilder<'r> {
    builder: LogUriBuilder<'r>,
}

impl<'r> RootLoggerUriBuilder<'r> {
    pub fn new(builder: LogUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn level(self, level: &str) -> TerminalUriBuilder<Self> {
        TerminalUriBuilder::new(self, level.to_string())
    }
}

impl<'r> UriBuilder for RootLoggerUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/rootLogger", self.builder.build()?);
        Ok(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::{self, TEST_HOST};

    fn base_uri() -> String {
        format!("{}/logs", tests::base_uri())
    }

    fn builder<'a>() -> LogUriBuilder<'a> {
        ResourceUriBuilder::default().host(TEST_HOST).logs()
    }

    #[test]
    fn logs_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, base_uri());
    }

    #[test]
    fn logger_uri_works() {
        let uri = builder().logger("test").build();
        assert_uri!(uri, format!("{}/logger/test", base_uri()));
    }

    #[test]
    fn logger_level_uri_works() {
        let uri = builder().logger("test").level("DEBUG").build();
        assert_uri!(uri, format!("{}/logger/test/DEBUG", base_uri()));
    }

    #[test]
    fn root_logger_uri_works() {
        let uri = builder().root_logger().build();
        assert_uri!(uri, format!("{}/rootLogger", base_uri()));
    }

    #[test]
    fn root_logger_level_uri_works() {
        let uri = builder().root_logger().level("DEBUG").build();
        assert_uri!(uri, format!("{}/rootLogger/DEBUG", base_uri()));
    }
}
