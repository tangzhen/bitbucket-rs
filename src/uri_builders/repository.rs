use crate::uri_builders::{WithProjectResourceUriBuilder, UriBuilder, BuildResult, TerminalUriBuilder};
use serde::Serialize;
use serde_plain;

#[derive(Debug)]
pub struct RepositoryResourceUriBuilder<'r> {
    builder: WithProjectResourceUriBuilder<'r>,
    repo: Option<&'r str>,
}

impl<'r> RepositoryResourceUriBuilder<'r> {
    pub fn new(builder: WithProjectResourceUriBuilder<'r>) -> Self {
        Self { builder, repo: None }
    }

    pub fn repository(mut self, repo: &'r str) -> WithRepositoryResourceUriBuilder<'r> {
        self.repo = Some(repo);
        WithRepositoryResourceUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for RepositoryResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/repos", self.builder.build()?);
        let uri = match &self.repo {
            None => uri,
            Some(repo) => format!("{}/{}", uri, repo)
        };

        Ok(uri)
    }
}

#[derive(Debug, Serialize)]
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

pub struct WithRepositoryResourceUriBuilder<'r> {
    builder: RepositoryResourceUriBuilder<'r>,
    action: Option<RepositoryAction>,
}

impl<'r> WithRepositoryResourceUriBuilder<'r> {
    pub fn new(builder: RepositoryResourceUriBuilder<'r>) -> Self {
        Self { builder, action: None }
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

    // TODO: This needs another type
    pub fn branches(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Branches);
        TerminalUriBuilder::new(self)
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

    // TODO: This needs another type
    pub fn commits(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Commits);
        TerminalUriBuilder::new(self)
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

    // TODO: This needs another type
    pub fn pull_requests(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::PullRequests);
        TerminalUriBuilder::new(self)
    }

    pub fn tags(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(RepositoryAction::Tags);
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for WithRepositoryResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = self.builder.build()?;
        let uri = match &self.action {
            None => uri,
            Some(action) => {
                let action = serde_plain::to_string(&action).unwrap();
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
    use crate::uri_builders::tests::TEST_HOST;

    #[test]
    fn repository_resource_uri_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project("TEST")
            .repos()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/projects/TEST/repos");
    }
}