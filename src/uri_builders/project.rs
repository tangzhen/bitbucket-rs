use crate::uri_builders::{ResourceUriBuilder, UriBuilder, BuildResult, RepositoryUriBuilder};

#[derive(Debug, Clone)]
pub struct ProjectUriBuilder<'r> {
    builder: ResourceUriBuilder<'r>,
}

impl<'r> ProjectUriBuilder<'r> {
    pub fn new(builder: ResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn project(self, project: &'r str) -> WithProjectUriBuilder<'r> {
        WithProjectUriBuilder::new(self, project)
    }
}

impl<'r> UriBuilder for ProjectUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/projects", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithProjectUriBuilder<'r> {
    builder: ProjectUriBuilder<'r>,
    project: &'r str,
}

impl<'r> WithProjectUriBuilder<'r> {
    pub fn new(builder: ProjectUriBuilder<'r>, project: &'r str) -> Self {
        Self { builder, project }
    }

    pub fn avatar(self) -> ProjectAvatarUriBuilder<'r> {
        ProjectAvatarUriBuilder::new(self)
    }

    pub fn repos(self) -> RepositoryUriBuilder<'r> {
        RepositoryUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for WithProjectUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.project);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct ProjectAvatarUriBuilder<'r> {
    builder: WithProjectUriBuilder<'r>
}

impl<'r> ProjectAvatarUriBuilder<'r> {
    pub fn new(builder: WithProjectUriBuilder<'r>) -> Self {
        Self { builder }
    }
}

impl<'r> UriBuilder for ProjectAvatarUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        Ok(format!("{}/avatar.png", self.builder.build()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::{TEST_HOST, TEST_PROJECT};

    fn base_uri() -> String {
        format!("{}/projects", crate::uri_builders::tests::base_uri())
    }

    fn builder<'a>() -> WithProjectUriBuilder<'a> {
        ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
    }

    #[test]
    fn project_resource_uri_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).projects().build();
        assert_uri!(uri, base_uri());
    }

    #[test]
    fn with_project_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, format!("{}/{}", base_uri(), TEST_PROJECT));
    }

    #[test]
    fn with_project_avatar_works() {
        let uri = builder().avatar().build();
        assert_uri!(uri, format!("{}/{}/avatar.png", base_uri(), TEST_PROJECT));
    }
}