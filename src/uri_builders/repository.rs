use crate::uri_builders::{
    WithProjectUriBuilder,
    UriBuilder,
    BuildResult,
    BranchUriBuilder,
    CommitUriBuilder,
    PullRequestUriBuilder,
    DiffUriBuilder,
    PermissionUriBuilder,
};
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

    terminal_resource_fn!(forks);
    terminal_resource_fn!(recreate);
    terminal_resource_fn!(related);
    terminal_resource_fn!(browse);      // TODO: separate type
    terminal_resource_fn!(changes);
    terminal_resource_fn!(files);       // TODO: separate type
    terminal_resource_fn!(tags);

    pub fn branches(self) -> BranchUriBuilder<'r> {
        BranchUriBuilder::new(self)
    }

    pub fn commits(self) -> CommitUriBuilder<'r> {
        CommitUriBuilder::new(self)
    }

    pub fn diff(self) -> DiffUriBuilder<Self> {
        DiffUriBuilder::new(self)
    }

    pub fn pull_requests(self) -> PullRequestUriBuilder<'r> {
        PullRequestUriBuilder::new(self)
    }

    pub fn permissions(self) -> PermissionUriBuilder<Self> {
        PermissionUriBuilder::new(self)
    }

    pub fn compare(self) -> CompareRepositoryUriBuilder<'r> {
        CompareRepositoryUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for WithRepositoryUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.repo);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct CompareRepositoryUriBuilder<'r> {
    builder: WithRepositoryUriBuilder<'r>
}

impl<'r> CompareRepositoryUriBuilder<'r> {
    pub fn new(builder: WithRepositoryUriBuilder<'r>) -> Self {
        Self { builder }
    }

    terminal_resource_fn!(changes);
    terminal_resource_fn!(commits);

    pub fn diff(self) -> DiffUriBuilder<Self> {
        DiffUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for CompareRepositoryUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/compare", self.builder.build()?);
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
    fn repo_related_uri_works() {
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

    #[test]
    fn repo_compare_uri_works() {
        let uri = builder().compare().build();
        assert_uri!(uri, format_repo_uri("compare"));
    }
}
