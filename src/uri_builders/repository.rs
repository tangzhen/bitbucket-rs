use crate::uri_builders::{
    BranchUriBuilder, BrowseUriBuilder, BuildResult, CommitUriBuilder, DiffUriBuilder,
    FileUriBuilder, PermissionUriBuilder, PullRequestUriBuilder, UriBuilder, WithProjectUriBuilder,
};

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
    terminal_resource_fn!(changes);
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

    pub fn browse(self) -> BrowseUriBuilder<Self> {
        BrowseUriBuilder::new(self)
    }

    pub fn files(self) -> FileUriBuilder<Self> {
        FileUriBuilder::new(self)
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

    pub fn settings(self) -> RepositorySettingsUriBuilder<'r> {
        RepositorySettingsUriBuilder::new(self)
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
    builder: WithRepositoryUriBuilder<'r>,
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

#[derive(Debug, Clone)]
pub struct RepositorySettingsUriBuilder<'r> {
    builder: WithRepositoryUriBuilder<'r>,
}

impl<'r> RepositorySettingsUriBuilder<'r> {
    pub fn new(builder: WithRepositoryUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn hooks(self) -> RepoHookSettingsUriBuilder<'r> {
        RepoHookSettingsUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for RepositorySettingsUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/settings", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct RepoHookSettingsUriBuilder<'r> {
    builder: RepositorySettingsUriBuilder<'r>,
}

impl<'r> RepoHookSettingsUriBuilder<'r> {
    pub fn new(builder: RepositorySettingsUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn hook(self, hook: &'r str) -> WithHookUriBuilder<'r> {
        WithHookUriBuilder::new(self, hook)
    }
}

impl<'r> UriBuilder for RepoHookSettingsUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/hooks", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithHookUriBuilder<'r> {
    builder: RepoHookSettingsUriBuilder<'r>,
    hook: &'r str,
}

impl<'r> WithHookUriBuilder<'r> {
    pub fn new(builder: RepoHookSettingsUriBuilder<'r>, hook: &'r str) -> Self {
        Self { builder, hook }
    }

    terminal_resource_fn!(enabled);
    terminal_resource_fn!(settings);
}

impl<'r> UriBuilder for WithHookUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.hook);
        Ok(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::{TEST_HOST, TEST_PROJECT, TEST_REPO};
    use crate::uri_builders::ResourceUriBuilder;

    fn base_uri() -> String {
        format!(
            "{}/projects/{}/repos",
            crate::uri_builders::tests::base_uri(),
            TEST_PROJECT
        )
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

    #[test]
    fn repo_settings_uri_works() {
        let uri = builder().settings().build();
        assert_uri!(uri, format_repo_uri("settings"));
    }

    #[test]
    fn repo_hook_settings_uri_works() {
        let uri = builder().settings().hooks().build();
        assert_uri!(uri, format_repo_uri("settings/hooks"));
    }

    #[test]
    fn with_hook_settings_uri_works() {
        let uri = builder().settings().hooks().hook("test-hook").build();
        assert_uri!(uri, format_repo_uri("settings/hooks/test-hook"));
    }

    #[test]
    fn with_hook_enabled_settings_uri_works() {
        let uri = builder().settings().hooks().hook("test-hook").enabled().build();
        assert_uri!(uri, format_repo_uri("settings/hooks/test-hook/enabled"));
    }

    #[test]
    fn hook_settings_uri_works() {
        let uri = builder().settings().hooks().hook("test-hook").settings().build();
        assert_uri!(uri, format_repo_uri("settings/hooks/test-hook/settings"));
    }
}
