use crate::uri_builders::{BuildResult, ResourceUriBuilder, TerminalUriBuilder, UriBuilder};

#[derive(Debug, Clone)]
pub struct UserUriBuilder<'r> {
    builder: ResourceUriBuilder<'r>,
}

impl<'r> UserUriBuilder<'r> {
    pub fn new(builder: ResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn user(self, user: &'r str) -> WithUserUriBuilder<'r> {
        WithUserUriBuilder::new(self, user)
    }

    terminal_resource_fn!(credentials);
}

impl<'r> UriBuilder for UserUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/users", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct WithUserUriBuilder<'r> {
    builder: UserUriBuilder<'r>,
    user: &'r str,
}

impl<'r> WithUserUriBuilder<'r> {
    pub fn new(builder: UserUriBuilder<'r>, user: &'r str) -> Self {
        Self { builder, user }
    }

    pub fn avatar(self) -> TerminalUriBuilder<Self> {
        TerminalUriBuilder::new(self, "avatar.png".to_string())
    }
}

impl<'r> UriBuilder for WithUserUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/{}", self.builder.build()?, self.user);
        Ok(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::{self, TEST_HOST};

    fn builder<'a>() -> UserUriBuilder<'a> {
        ResourceUriBuilder::default().host(TEST_HOST).users()
    }

    fn base_uri() -> String {
        format!("{}/users", tests::base_uri())
    }

    #[test]
    fn user_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, base_uri());
    }

    #[test]
    fn user_credentials_uri_works() {
        let uri = builder().credentials().build();
        assert_uri!(uri, format!("{}/credentials", base_uri()));
    }

    #[test]
    fn with_user_uri_works() {
        let uri = builder().user("george").build();
        assert_uri!(uri, format!("{}/george", base_uri()));
    }

    #[test]
    fn user_avatar_uri_works() {
        let uri = builder().user("george").avatar().build();
        assert_uri!(uri, format!("{}/george/avatar.png", base_uri()));
    }
}
