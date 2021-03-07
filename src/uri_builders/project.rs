use crate::uri_builders::{
    BuildResult, GroupPermissionUriBuilder, PermissionUriBuilder, RepositoryUriBuilder,
    ResourceUriBuilder, UriBuilder, UserPermissionUriBuilder,
};

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

    pub fn permissions(self) -> ProjectPermissionsUriBuilder<'r> {
        ProjectPermissionsUriBuilder::new(self)
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
    builder: WithProjectUriBuilder<'r>,
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

#[derive(Debug, Clone)]
pub struct ProjectPermissionsUriBuilder<'r> {
    builder: PermissionUriBuilder<WithProjectUriBuilder<'r>>,
}

impl<'r> ProjectPermissionsUriBuilder<'r> {
    pub fn new(builder: WithProjectUriBuilder<'r>) -> Self {
        let builder = PermissionUriBuilder::new(builder);
        Self { builder }
    }

    pub fn groups(self) -> GroupPermissionUriBuilder<WithProjectUriBuilder<'r>> {
        self.builder.groups()
    }

    pub fn users(self) -> UserPermissionUriBuilder<WithProjectUriBuilder<'r>> {
        self.builder.users()
    }

    pub fn permission(self, perm: &'r str) -> WithProjectPermissionUriBuilder<'r> {
        WithProjectPermissionUriBuilder::new(self, perm)
    }
}

impl<'r> UriBuilder for ProjectPermissionsUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        self.builder.build()
    }
}

#[derive(Debug, Clone)]
pub struct WithProjectPermissionUriBuilder<'r> {
    builder: ProjectPermissionsUriBuilder<'r>,
    permission: &'r str,
}

impl<'r> WithProjectPermissionUriBuilder<'r> {
    pub fn new(builder: ProjectPermissionsUriBuilder<'r>, permission: &'r str) -> Self {
        Self {
            builder,
            permission,
        }
    }

    terminal_resource_fn!(all);
}

impl<'r> UriBuilder for WithProjectPermissionUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.permission);
        Ok(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::{self, TEST_HOST, TEST_PROJECT};

    fn base_uri() -> String {
        format!("{}/projects/{}", tests::base_uri(), TEST_PROJECT)
    }

    fn builder<'a>() -> WithProjectUriBuilder<'a> {
        ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
    }

    #[test]
    fn project_resource_uri_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .build();
        assert_uri!(uri, format!("{}/projects", tests::base_uri()));
    }

    #[test]
    fn with_project_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, base_uri());
    }

    #[test]
    fn with_project_avatar_works() {
        let uri = builder().avatar().build();
        assert_uri!(uri, format!("{}/avatar.png", base_uri()));
    }

    #[test]
    fn project_permissions_uri_works() {
        let uri = builder().permissions().build();
        assert_uri!(uri, format!("{}/permissions", base_uri()));
    }

    #[test]
    fn project_group_permissions_uri_works() {
        let uri = builder().permissions().groups().build();
        assert_uri!(uri, format!("{}/permissions/groups", base_uri()));
    }

    #[test]
    fn project_user_permissions_uri_works() {
        let uri = builder().permissions().users().build();
        assert_uri!(uri, format!("{}/permissions/users", base_uri()));
    }

    #[test]
    fn with_project_permission_uri_works() {
        let uri = builder().permissions().permission("REPO_READ").build();
        assert_uri!(uri, format!("{}/permissions/REPO_READ", base_uri()));
    }

    #[test]
    fn with_project_permission_all_uri_works() {
        let uri = builder()
            .permissions()
            .permission("REPO_READ")
            .all()
            .build();
        assert_uri!(uri, format!("{}/permissions/REPO_READ/all", base_uri()));
    }
}
