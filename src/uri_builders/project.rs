use crate::uri_builders::{ResourceUriBuilder, UriBuilder, BuildResult, RepositoryResourceUriBuilder};

#[derive(Debug, Clone)]
pub struct ProjectResourceUriBuilder<'r> {
    builder: ResourceUriBuilder<'r>,
}

impl<'r> ProjectResourceUriBuilder<'r> {
    pub fn new(builder: ResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn project(self, project: &'r str) -> WithProjectResourceUriBuilder<'r> {
        WithProjectResourceUriBuilder::new(self, project)
    }
}

impl<'r> UriBuilder for ProjectResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/projects", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithProjectResourceUriBuilder<'r> {
    builder: ProjectResourceUriBuilder<'r>,
    project: &'r str,
}

impl<'r> WithProjectResourceUriBuilder<'r> {
    pub fn new(builder: ProjectResourceUriBuilder<'r>, project: &'r str) -> Self {
        Self { builder, project }
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
        let uri = format!("{}/{}", self.builder.build()?, self.project);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
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
    use crate::uri_builders::tests::{TEST_HOST, TEST_PROJECT};

    fn base_uri() -> String {
        format!("{}/projects", crate::uri_builders::tests::base_uri())
    }

    fn builder<'a>() -> WithProjectResourceUriBuilder<'a> {
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