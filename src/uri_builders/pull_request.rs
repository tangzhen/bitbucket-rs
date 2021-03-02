use crate::uri_builders::{WithRepositoryResourceUriBuilder, UriBuilder, BuildResult, TerminalUriBuilder};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct PullRequestResourceUriBuilder<'r> {
    builder: WithRepositoryResourceUriBuilder<'r>,
}

impl<'r> PullRequestResourceUriBuilder<'r> {
    pub fn new(builder: WithRepositoryResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn pull_request(self, pull_request_id: u64) -> WithPullRequestResourceUriBuilder<'r> {
        WithPullRequestResourceUriBuilder::new(self, pull_request_id)
    }
}

impl<'r> UriBuilder for PullRequestResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        Ok(self.builder.build()?)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
enum PullRequestAction {
    Activities,
    Decline,
    Merge,
    Reopen,
    Approve,
    Changes,
    Comments,
    Commits,
    Diff,
    Participants,
    Tasks,
    Watch,
}

#[derive(Debug, Clone)]
pub struct WithPullRequestResourceUriBuilder<'r> {
    builder: PullRequestResourceUriBuilder<'r>,
    id: u64,
    action: Option<PullRequestAction>,
}

impl<'r> WithPullRequestResourceUriBuilder<'r> {
    pub fn new(builder: PullRequestResourceUriBuilder<'r>, id: u64) -> Self {
        Self { builder, id, action: None }
    }

    pub fn activities(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Activities);
        TerminalUriBuilder::new(self)
    }

    pub fn decline(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Decline);
        TerminalUriBuilder::new(self)
    }

    pub fn merge(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Merge);
        TerminalUriBuilder::new(self)
    }

    pub fn reopen(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Reopen);
        TerminalUriBuilder::new(self)
    }

    pub fn approve(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Approve);
        TerminalUriBuilder::new(self)
    }

    pub fn changes(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Changes);
        TerminalUriBuilder::new(self)
    }

    pub fn comments(mut self) -> PullRequestCommentResourceUriBuilder<'r> {
        self.action = Some(PullRequestAction::Comments);
        PullRequestCommentResourceUriBuilder::new(self)
    }

    pub fn commits(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Commits);
        TerminalUriBuilder::new(self)
    }

    // TODO: This needs a separate type
    pub fn diff(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Diff);
        TerminalUriBuilder::new(self)
    }

    pub fn participants(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Participants);
        TerminalUriBuilder::new(self)
    }

    // TODO: This needs a separate type
    pub fn tasks(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Tasks);
        TerminalUriBuilder::new(self)
    }

    pub fn watch(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(PullRequestAction::Watch);
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for WithPullRequestResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.id);
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

#[derive(Debug, Clone)]
pub struct PullRequestCommentResourceUriBuilder<'r> {
    builder: WithPullRequestResourceUriBuilder<'r>
}

impl<'r> PullRequestCommentResourceUriBuilder<'r> {
    pub fn new(builder: WithPullRequestResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn comment(self, comment_id: u64) -> WithPullRequestCommentResourceUriBuilder<'r> {
        WithPullRequestCommentResourceUriBuilder::new(self, comment_id)
    }
}

impl<'r> UriBuilder for PullRequestCommentResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        Ok(self.builder.build()?)
    }
}

#[derive(Debug, Clone)]
pub struct WithPullRequestCommentResourceUriBuilder<'r> {
    builder: PullRequestCommentResourceUriBuilder<'r>,
    comment_id: u64,
}

impl<'r> WithPullRequestCommentResourceUriBuilder<'r> {
    pub fn new(builder: PullRequestCommentResourceUriBuilder<'r>, comment_id: u64) -> Self {
        Self { builder, comment_id }
    }
}

impl<'r> UriBuilder for WithPullRequestCommentResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.comment_id);
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
            "{}/projects/{}/repos/{}/pull-requests",
            crate::uri_builders::tests::base_uri(),
            TEST_PROJECT,
            TEST_REPO
        )
    }

    fn builder<'a>() -> WithPullRequestResourceUriBuilder<'a> {
        ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
            .repos()
            .repository(TEST_REPO)
            .pull_requests()
            .pull_request(1)
    }

    #[test]
    fn pull_request_uri_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
            .repos()
            .repository(TEST_REPO)
            .pull_requests()
            .build();

        assert_uri!(uri, base_uri());
    }

    #[test]
    fn with_pull_request_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, format!("{}/1", base_uri()));
    }

    #[test]
    fn with_pull_request_activities_uri_works() {
        let uri = builder().activities().build();
        assert_uri!(uri, format!("{}/1/activities", base_uri()));
    }

    #[test]
    fn with_pull_request_decline_uri_works() {
        let uri = builder().decline().build();
        assert_uri!(uri, format!("{}/1/decline", base_uri()));
    }

    #[test]
    fn with_pull_request_merge_uri_works() {
        let uri = builder().merge().build();
        assert_uri!(uri, format!("{}/1/merge", base_uri()));
    }

    #[test]
    fn with_pull_request_reopen_uri_works() {
        let uri = builder().reopen().build();
        assert_uri!(uri, format!("{}/1/reopen", base_uri()));
    }

    #[test]
    fn with_pull_request_approve_uri_works() {
        let uri = builder().approve().build();
        assert_uri!(uri, format!("{}/1/approve", base_uri()));
    }

    #[test]
    fn with_pull_request_changes_uri_works() {
        let uri = builder().changes().build();
        assert_uri!(uri, format!("{}/1/changes", base_uri()));
    }

    #[test]
    fn with_pull_request_comments_uri_works() {
        let uri = builder().comments().build();
        assert_uri!(uri, format!("{}/1/comments", base_uri()));
    }

    #[test]
    fn with_comment_uri_works() {
        let uri = builder().comments().comment(1).build();
        assert_uri!(uri, format!("{}/1/comments/1", base_uri()));
    }

    #[test]
    fn with_pull_request_commits_uri_works() {
        let uri = builder().commits().build();
        assert_uri!(uri, format!("{}/1/commits", base_uri()));
    }

    #[test]
    fn with_pull_request_diff_uri_works() {
        let uri = builder().diff().build();
        assert_uri!(uri, format!("{}/1/diff", base_uri()));
    }

    #[test]
    fn with_pull_request_participants_uri_works() {
        let uri = builder().participants().build();
        assert_uri!(uri, format!("{}/1/participants", base_uri()));
    }

    #[test]
    fn with_pull_request_tasks_uri_works() {
        let uri = builder().tasks().build();
        assert_uri!(uri, format!("{}/1/tasks", base_uri()));
    }

    #[test]
    fn with_pull_request_watch_uri_works() {
        let uri = builder().watch().build();
        assert_uri!(uri, format!("{}/1/watch", base_uri()));
    }
}