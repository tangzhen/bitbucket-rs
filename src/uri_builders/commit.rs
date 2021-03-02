use crate::uri_builders::{WithRepositoryUriBuilder, UriBuilder, BuildResult, TerminalUriBuilder};
use serde::Serialize;
use serde_plain;
use crate::models::get::Commit;

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
        Ok(self.builder.build()?)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
enum CommitAction {
    Changes,
    Comments,
    Diff,
    Watch,
}

#[derive(Debug, Clone)]
pub struct WithCommitUriBuilder<'r> {
    builder: CommitUriBuilder<'r>,
    commit_id: &'r str,
    action: Option<CommitAction>,
}

impl<'r> WithCommitUriBuilder<'r> {
    pub fn new(builder: CommitUriBuilder<'r>, commit_id: &'r str) -> Self {
        Self { builder, commit_id, action: None }
    }

    pub fn changes(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(CommitAction::Changes);
        TerminalUriBuilder::new(self)
    }

    // TODO: This need a different type
    pub fn comments(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(CommitAction::Comments);
        TerminalUriBuilder::new(self)
    }

    // TODO: This need a different type
    pub fn diff(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(CommitAction::Diff);
        TerminalUriBuilder::new(self)
    }

    pub fn watch(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(CommitAction::Watch);
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for WithCommitUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.commit_id);
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
    use crate::uri_builders::tests::{TEST_PROJECT, TEST_REPO, TEST_HOST};
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

    fn commit_id() -> &'static str { "76bf028" }

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