use crate::uri_builders::{ResourceUriBuilder, UriBuilder, BuildResult, RepositoryResourceUriBuilder};

#[derive(Debug)]
pub struct ProjectResourceUriBuilder<'r> {
    builder: ResourceUriBuilder<'r>,
    project: Option<&'r str>,
}

impl<'r> ProjectResourceUriBuilder<'r> {
    pub fn new(builder: ResourceUriBuilder<'r>) -> Self {
        Self { builder, project: None }
    }

    pub fn project(mut self, project: &'r str) -> WithProjectResourceUriBuilder<'r> {
        self.project = Some(project);
        WithProjectResourceUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for ProjectResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/projects", self.builder.build()?);
        let uri = match &self.project {
            None => uri,
            Some(project) => format!("{}/{}", uri, project)
        };

        Ok(uri)
    }
}

#[derive(Debug)]
pub struct WithProjectResourceUriBuilder<'r> {
    builder: ProjectResourceUriBuilder<'r>
}

impl<'r> WithProjectResourceUriBuilder<'r> {
    pub fn new(builder: ProjectResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn avatar(self) -> ProjectAvatarResourceUriBuilder<'r> {
        ProjectAvatarResourceUriBuilder::new(self)
    }

    pub fn repos(self) -> RepositoryResourceUriBuilder<'r> {
        RepositoryResourceUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for WithProjectResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        Ok(self.builder.build()?)
    }
}

#[derive(Debug)]
pub struct ProjectAvatarResourceUriBuilder<'r> {
    builder: WithProjectResourceUriBuilder<'r>
}

impl<'r> ProjectAvatarResourceUriBuilder<'r> {
    pub fn new(builder: WithProjectResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }
}

impl<'r> UriBuilder for ProjectAvatarResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        Ok(format!("{}/avatar.png", self.builder.build()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::TEST_HOST;

    #[test]
    fn project_resource_uri_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).projects().build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/projects");
    }

    #[test]
    fn with_project_uri_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project("TEST")
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/projects/TEST")
    }

    #[test]
    fn with_project_avatar_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project("TEST")
            .avatar()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/projects/TEST/avatar.png");
    }
}