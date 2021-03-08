use crate::uri_builders::{
    BuildResult, DiffUriBuilder, TerminalUriBuilder, UriBuilder, WithRepositoryUriBuilder,
};

#[derive(Debug, Clone)]
pub struct CommitUriBuilder<'r> {
    builder: WithRepositoryUriBuilder<'r>,
}

impl<'r> CommitUriBuilder<'r> {
    pub fn new(builder: WithRepositoryUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn commit(self, commit_id: &'r str) -> WithCommitUriBuilder<'r> {
        WithCommitUriBuilder::new(self, commit_id)
    }
}

impl<'r> UriBuilder for CommitUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/commits", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithCommitUriBuilder<'r> {
    builder: CommitUriBuilder<'r>,
    commit_id: &'r str,
}

impl<'r> WithCommitUriBuilder<'r> {
    pub fn new(builder: CommitUriBuilder<'r>, commit_id: &'r str) -> Self {
        Self { builder, commit_id }
    }

    pub fn diff(self) -> DiffUriBuilder<Self> {
        DiffUriBuilder::new(self)
    }

    pub fn comments(self) -> CommitCommentUriBuilder<'r> {
        CommitCommentUriBuilder::new(self)
    }

    terminal_resource_fn!(changes);
    terminal_resource_fn!(watch);
}

impl<'r> UriBuilder for WithCommitUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.commit_id);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct CommitCommentUriBuilder<'r> {
    builder: WithCommitUriBuilder<'r>,
}

impl<'r> CommitCommentUriBuilder<'r> {
    pub fn new(builder: WithCommitUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn comment(self, comment_id: u64) -> TerminalUriBuilder<Self> {
        TerminalUriBuilder::new(self, comment_id.to_string())
    }
}

impl<'r> UriBuilder for CommitCommentUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/comments", self.builder.build()?);
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
            "{}/projects/{}/repos/{}/commits",
            crate::uri_builders::tests::base_uri(),
            TEST_PROJECT,
            TEST_REPO
        )
    }

    fn builder<'a>() -> WithCommitUriBuilder<'a> {
        ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
            .repos()
            .repository(TEST_REPO)
            .commits()
            .commit(commit_id())
    }

    fn commit_id() -> &'static str {
        "76bf028"
    }

    #[test]
    fn commit_uri_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
            .repos()
            .repository(TEST_REPO)
            .commits()
            .build();

        assert_uri!(uri, base_uri());
    }

    #[test]
    fn with_commit_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, format!("{}/{}", base_uri(), commit_id()));
    }

    #[test]
    fn commit_changes_works() {
        let uri = builder().changes().build();
        assert_uri!(uri, format!("{}/{}/changes", base_uri(), commit_id()));
    }

    #[test]
    fn commit_comments_works() {
        let uri = builder().comments().build();
        assert_uri!(uri, format!("{}/{}/comments", base_uri(), commit_id()));
    }

    #[test]
    fn commit_comment_id_uri_works() {
        let uri = builder().comments().comment(1).build();
        assert_uri!(uri, format!("{}/{}/comments/1", base_uri(), commit_id()));
    }

    #[test]
    fn commit_diff_works() {
        let uri = builder().diff().build();
        assert_uri!(uri, format!("{}/{}/diff", base_uri(), commit_id()));
    }

    #[test]
    fn commit_watch_works() {
        let uri = builder().watch().build();
        assert_uri!(uri, format!("{}/{}/watch", base_uri(), commit_id()));
    }
}
