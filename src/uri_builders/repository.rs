use crate::uri_builders::{WithProjectUriBuilder, UriBuilder, BuildResult, TerminalUriBuilder, BranchUriBuilder, CommitUriBuilder, PullRequestUriBuilder};
use serde::Serialize;
use serde_plain;

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

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
enum RepositoryAction {
    Forks,
    Recreate,
    Related,
    Branches,
    Browse,
    Changes,
    Commits,
    Compare,
    Diff,
    Files,
    Permissions,
    PullRequests,
    Tags,
}

#[derive(Debug, Clone)]
pub struct WithRepositoryUriBuilder<'r> {
    builder: RepositoryUriBuilder<'r>,
    repo: &'r str,
    action: Option<RepositoryAction>,
}

impl<'r> WithRepositoryUriBuilder<'r> {
    pub fn new(builder: RepositoryUriBuilder<'r>, repo: &'r str) -> Self {
        Self { builder, repo, action: None }
    }

    pub fn forks(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Forks);
        TerminalUriBuilder::new(self)
    }

    pub fn recreate(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Recreate);
        TerminalUriBuilder::new(self)
    }

    pub fn related(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Related);
        TerminalUriBuilder::new(self)
    }

    pub fn branches(mut self) -> BranchUriBuilder<'r> {
        self.action = Some(RepositoryAction::Branches);
        BranchUriBuilder::new(self)
    }

    // TODO: This needs another type
    pub fn browse(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Browse);
        TerminalUriBuilder::new(self)
    }

    pub fn changes(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Changes);
        TerminalUriBuilder::new(self)
    }

    pub fn commits(mut self) -> CommitUriBuilder<'r> {
        self.action = Some(RepositoryAction::Commits);
        CommitUriBuilder::new(self)
    }

    // TODO: This needs another type
    pub fn compare(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Compare);
        TerminalUriBuilder::new(self)
    }

    // TODO: This needs another type
    pub fn diff(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Diff);
        TerminalUriBuilder::new(self)
    }

    // TODO: This needs another type
    pub fn files(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Files);
        TerminalUriBuilder::new(self)
    }

    // TODO: This needs another type
    pub fn permissions(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Permissions);
        TerminalUriBuilder::new(self)
    }

    pub fn pull_requests(mut self) -> PullRequestUriBuilder<'r> {
        self.action = Some(RepositoryAction::PullRequests);
        PullRequestUriBuilder::new(self)
    }

    pub fn tags(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Tags);
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for WithRepositoryUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.repo);
        let uri = match &self.action {
            None => uri,
            Some(action) => {
                let action = serde_plain::to_string(action).unwrap();
                format!("{}/{}", uri, action)
            }
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
