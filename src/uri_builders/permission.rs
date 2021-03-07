use crate::uri_builders::{UriBuilder, BuildResult};

#[derive(Debug, Clone)]
pub struct PermissionUriBuilder<B> {
    builder: B
}

impl<B> PermissionUriBuilder<B> where B: UriBuilder {
    pub fn new(builder: B) -> Self {
        Self { builder }
    }

    pub fn groups(self) -> GroupPermissionUriBuilder<B> {
        GroupPermissionUriBuilder::new(self)
    }

    pub fn users(self) -> UserPermissionUriBuilder<B> {
        UserPermissionUriBuilder::new(self)
    }
}

impl<B> UriBuilder for PermissionUriBuilder<B> where B: UriBuilder {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/permissions", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct GroupPermissionUriBuilder<B> {
    builder: PermissionUriBuilder<B>
}

impl<B> GroupPermissionUriBuilder<B> where B: UriBuilder {
    pub fn new(builder: PermissionUriBuilder<B>) -> Self {
        Self { builder }
    }

    terminal_resource_fn!(none);
}

impl<B> UriBuilder for GroupPermissionUriBuilder<B> where B: UriBuilder {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/groups", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct UserPermissionUriBuilder<B> {
    builder: PermissionUriBuilder<B>
}

impl<B> UserPermissionUriBuilder<B> where B: UriBuilder {
    pub fn new(builder: PermissionUriBuilder<B>) -> Self {
        Self { builder }
    }

    terminal_resource_fn!(none);
}

impl<B> UriBuilder for UserPermissionUriBuilder<B> where B: UriBuilder {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/users", self.builder.build()?);
        Ok(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EmptyUriBuilder;

    impl UriBuilder for EmptyUriBuilder {
        fn build(&self) -> BuildResult {
            Ok(String::new())
        }
    }

    fn builder() -> PermissionUriBuilder<EmptyUriBuilder> {
        PermissionUriBuilder::new(EmptyUriBuilder)
    }

    #[test]
    fn permissions_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, "/permissions");
    }

    #[test]
    fn group_permissions_uri_works() {
        let uri = builder().groups().build();
        assert_uri!(uri, "/permissions/groups");
    }

    #[test]
    fn none_group_permissions_uri_works() {
        let uri = builder().groups().none().build();
        assert_uri!(uri, "/permissions/groups/none");
    }

    #[test]
    fn user_permissions_uri_works() {
        let uri = builder().users().build();
        assert_uri!(uri, "/permissions/users");
    }

    #[test]
    fn none_user_permissions_uri_works() {
        let uri = builder().users().none().build();
        assert_uri!(uri, "/permissions/users/none");
    }
}