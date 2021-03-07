use crate::uri_builders::{BuildResult, DiffUriBuilder, UriBuilder, WithRepositoryUriBuilder};

#[derive(Debug, Clone)]
pub struct PullRequestUriBuilder<'r> {
    builder: WithRepositoryUriBuilder<'r>,
}

impl<'r> PullRequestUriBuilder<'r> {
    pub fn new(builder: WithRepositoryUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn pull_request(self, pull_request_id: u64) -> WithPullRequestUriBuilder<'r> {
        WithPullRequestUriBuilder::new(self, pull_request_id)
    }
}

impl<'r> UriBuilder for PullRequestUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/pull-requests", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithPullRequestUriBuilder<'r> {
    builder: PullRequestUriBuilder<'r>,
    id: u64,
}

impl<'r> WithPullRequestUriBuilder<'r> {
    pub fn new(builder: PullRequestUriBuilder<'r>, id: u64) -> Self {
        Self { builder, id }
    }

    pub fn diff(self) -> DiffUriBuilder<Self> {
        DiffUriBuilder::new(self)
    }

    pub fn comments(self) -> PullRequestCommentUriBuilder<'r> {
        PullRequestCommentUriBuilder::new(self)
    }

    pub fn tasks(self) -> PullRequestTasksUriBuilder<'r> {
        PullRequestTasksUriBuilder::new(self)
    }

    terminal_resource_fn!(activities);
    terminal_resource_fn!(decline);
    terminal_resource_fn!(merge);
    terminal_resource_fn!(reopen);
    terminal_resource_fn!(approve);
    terminal_resource_fn!(changes);
    terminal_resource_fn!(commits);
    terminal_resource_fn!(participants);
    terminal_resource_fn!(watch);
}

impl<'r> UriBuilder for WithPullRequestUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.id);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct PullRequestCommentUriBuilder<'r> {
    builder: WithPullRequestUriBuilder<'r>,
}

impl<'r> PullRequestCommentUriBuilder<'r> {
    pub fn new(builder: WithPullRequestUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn comment(self, comment_id: u64) -> WithPullRequestCommentUriBuilder<'r> {
        WithPullRequestCommentUriBuilder::new(self, comment_id)
    }
}

impl<'r> UriBuilder for PullRequestCommentUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/comments", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithPullRequestCommentUriBuilder<'r> {
    builder: PullRequestCommentUriBuilder<'r>,
    comment_id: u64,
}

impl<'r> WithPullRequestCommentUriBuilder<'r> {
    pub fn new(builder: PullRequestCommentUriBuilder<'r>, comment_id: u64) -> Self {
        Self {
            builder,
            comment_id,
        }
    }
}

impl<'r> UriBuilder for WithPullRequestCommentUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.comment_id);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct PullRequestTasksUriBuilder<'r> {
    builder: WithPullRequestUriBuilder<'r>,
}

impl<'r> PullRequestTasksUriBuilder<'r> {
    pub fn new(builder: WithPullRequestUriBuilder<'r>) -> Self {
        Self { builder }
    }

    terminal_resource_fn!(count);
}

impl<'r> UriBuilder for PullRequestTasksUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/tasks", self.builder.build()?);
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
            "{}/projects/{}/repos/{}/pull-requests",
            crate::uri_builders::tests::base_uri(),
            TEST_PROJECT,
            TEST_REPO
        )
    }

    fn builder<'a>() -> WithPullRequestUriBuilder<'a> {
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
    fn with_pull_request_task_count_uri_works() {
        let uri = builder().tasks().count().build();
        assert_uri!(uri, format!("{}/1/tasks/count", base_uri()));
    }

    #[test]
    fn with_pull_request_watch_uri_works() {
        let uri = builder().watch().build();
        assert_uri!(uri, format!("{}/1/watch", base_uri()));
    }
}
