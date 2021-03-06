use crate::uri_builders::{WithProjectUriBuilder, UriBuilder, BuildResult, TerminalUriBuilder, BranchUriBuilder, CommitUriBuilder, PullRequestUriBuilder, DiffUriBuilder};
use function_name::named;

#[derive(Debug, Clone)]
pub struct RepositoryUriBuilder<'r> {
    builder: WithProjectUriBuilder<'r>,
}

impl<'r> RepositoryUriBuilder<'r> {
    pub fn new(builder: WithProjectUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn repository(self, repo: &'r str) -> WithRepositoryUriBuilder<'r> {
        WithRepositoryUriBuilder::new(self, repo)
    }
}

impl<'r> UriBuilder for RepositoryUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/repos", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithRepositoryUriBuilder<'r> {
    builder: RepositoryUriBuilder<'r>,
    repo: &'r str,
}

impl<'r> WithRepositoryUriBuilder<'r> {
    pub fn new(builder: RepositoryUriBuilder<'r>, repo: &'r str) -> Self {
        Self { builder, repo }
    }

    #[named]
    pub fn forks(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn recreate(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn related(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    pub fn branches(self) -> BranchUriBuilder<'r> {
        BranchUriBuilder::new(self)
    }

    // TODO: This needs another type
    #[named]
    pub fn browse(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn changes(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    pub fn commits(self) -> CommitUriBuilder<'r> {
        CommitUriBuilder::new(self)
    }

    // TODO: This needs another type
    #[named]
    pub fn compare(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    pub fn diff(self) -> DiffUriBuilder<Self> {
        DiffUriBuilder::new(self)
    }

    // TODO: This needs another type
    #[named]
    pub fn files(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    // TODO: This needs another type
    #[named]
    pub fn permissions(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    pub fn pull_requests(self) -> PullRequestUriBuilder<'r> {
        PullRequestUriBuilder::new(self)
    }

    #[named]
    pub fn tags(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }
}

impl<'r> UriBuilder for WithRepositoryUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.repo);
        Ok(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::ResourceUriBuilder;
    use crate::uri_builders::tests::{TEST_HOST, TEST_PROJECT, TEST_REPO};

    fn base_uri() -> String {
        format!("{}/projects/{}/repos", crate::uri_builders::tests::base_uri(), TEST_PROJECT)
    }

    fn builder<'a>() -> WithRepositoryUriBuilder<'a> {
        ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
            .repos()
            .repository(TEST_REPO)
    }

    fn format_repo_uri(path: &str) -> String {
        format!("{}/{}/{}", base_uri(), TEST_REPO, path)
    }

    #[test]
    fn repository_uri_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
            .repos()
            .build();

        assert_uri!(uri, base_uri());
    }

    #[test]
    fn with_repository_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, format!("{}/{}", base_uri(), TEST_REPO));
    }

    #[test]
    fn repo_forks_uri_works() {
        let uri = builder().forks().build();
        assert_uri!(uri, format_repo_uri("forks"));
    }

    #[test]
    fn repo_recreate_uri_works() {
        let uri = builder().recreate().build();
        assert_uri!(uri, format_repo_uri("recreate"));
    }

    #[test]
    fn repo_relate_uri_works() {
        let uri = builder().related().build();
        assert_uri!(uri, format_repo_uri("related"));
    }

    #[test]
    fn repo_changes_uri_works() {
        let uri = builder().changes().build();
        assert_uri!(uri, format_repo_uri("changes"));
    }

    #[test]
    fn repo_tags_uri_works() {
        let uri = builder().tags().build();
        assert_uri!(uri, format_repo_uri("tags"));
    }
}
