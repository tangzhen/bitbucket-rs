use crate::uri_builders::{WithRepositoryResourceUriBuilder, TerminalUriBuilder, UriBuilder, BuildResult};

#[derive(Debug, Clone)]
pub struct BranchResourceUriBuilder<'r> {
    builder: WithRepositoryResourceUriBuilder<'r>,
    default: bool,
}

impl<'r> BranchResourceUriBuilder<'r> {
    pub fn new(builder: WithRepositoryResourceUriBuilder<'r>) -> Self {
        Self { builder, default: false }
    }

    pub fn default(mut self) -> TerminalUriBuilder<Self> {
        self.default = true;
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for BranchResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = self.builder.build()?;
        let uri = if self.default {
            format!("{}/default", uri)
        } else {
            uri
        };

        Ok(uri)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::ResourceUriBuilder;
    use crate::uri_builders::tests::{TEST_HOST, TEST_PROJECT, TEST_REPO};

    fn base_uri() -> String {
        format!(
            "{}/projects/{}/repos/{}/branches",
            crate::uri_builders::tests::base_uri(),
            TEST_PROJECT,
            TEST_REPO
        )
    }

    fn builder<'a>() -> BranchResourceUriBuilder<'a> {
        ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
            .repos()
            .repository(TEST_REPO)
            .branches()
    }

    #[test]
    fn branch_resource_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, base_uri());
    }

    #[test]
    fn branch_default_uri_works() {
        let uri = builder().default().build();
        assert_uri!(uri, format!("{}/default", base_uri()));
    }
}