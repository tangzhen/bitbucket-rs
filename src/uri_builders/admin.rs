use crate::uri_builders::{ResourceUriBuilder, UriBuilder, BuildResult, PermissionUriBuilder};
use function_name::named;

#[derive(Debug, Clone)]
pub struct AdminUriBuilder<'r> {
    builder: ResourceUriBuilder<'r>,
}

impl<'r> AdminUriBuilder<'r> {
    pub fn new(builder: ResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn groups(self) -> AdminGroupUriBuilder<'r> {
        AdminGroupUriBuilder::new(self)
    }

    pub fn users(self) -> AdminUserUriBuilder<'r> {
        AdminUserUriBuilder::new(self)
    }

    pub fn permissions(self) -> PermissionUriBuilder<Self> {
        PermissionUriBuilder::new(self)
    }

    pub fn mail_server(self) -> AdminMailServerUriBuilder<'r> {
        AdminMailServerUriBuilder::new(self)
    }

    terminal_resource_fn!(cluster);
    terminal_resource_fn!(licence);
}

impl<'r> UriBuilder for AdminUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/admin", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct AdminGroupUriBuilder<'r> {
    builder: AdminUriBuilder<'r>,
}

impl<'r> AdminGroupUriBuilder<'r> {
    pub fn new(builder: AdminUriBuilder<'r>) -> Self {
        Self { builder }
    }

    terminal_resource_fn!(add_user);
    terminal_resource_fn!(add_users);
    terminal_resource_fn!(more_members);
    terminal_resource_fn!(more_non_members);
    terminal_resource_fn!(remove_user);
}

impl<'r> UriBuilder for AdminGroupUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/groups", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct AdminUserUriBuilder<'r> {
    builder: AdminUriBuilder<'r>,
}

impl<'r> AdminUserUriBuilder<'r> {
    pub fn new(builder: AdminUriBuilder<'r>) -> Self {
        Self { builder }
    }

    terminal_resource_fn!(add_group);
    terminal_resource_fn!(add_groups);
    terminal_resource_fn!(captcha);
    terminal_resource_fn!(credentials);
    terminal_resource_fn!(more_members);
    terminal_resource_fn!(more_non_members);
    terminal_resource_fn!(remove_group);
    terminal_resource_fn!(rename);
}

impl<'r> UriBuilder for AdminUserUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/users", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct AdminMailServerUriBuilder<'r> {
    builder: AdminUriBuilder<'r>,
}

impl<'r> AdminMailServerUriBuilder<'r> {
    pub fn new(builder: AdminUriBuilder<'r>) -> Self {
        Self { builder }
    }

    terminal_resource_fn!(sender_address);
}

impl<'r> UriBuilder for AdminMailServerUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/mail-server", self.builder.build()?);
        Ok(uri)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::TEST_HOST;

    fn base_uri() -> String {
        format!("{}/admin", crate::uri_builders::tests::base_uri())
    }

    fn builder<'a>() -> AdminUriBuilder<'a> {
        ResourceUriBuilder::default().host(TEST_HOST).admin()
    }

    #[test]
    fn admin_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, base_uri());
    }

    #[test]
    fn admin_cluster_works() {
        let uri = builder().cluster().build();
        assert_uri!(uri, format!("{}/cluster", base_uri()));
    }

    #[test]
    fn admin_licence_works() {
        let uri = builder().licence().build();
        assert_uri!(uri, format!("{}/licence", base_uri()));
    }

    #[test]
    fn admin_groups_uri_works() {
        let uri = builder().groups().build();
        assert_uri!(uri, format!("{}/groups", base_uri()));
    }

    #[test]
    fn admin_groups_add_user_works() {
        let uri = builder().groups().add_user().build();
        assert_uri!(uri, format!("{}/groups/add-user", base_uri()));
    }

    #[test]
    fn admin_groups_add_users_works() {
        let uri = builder().groups().add_users().build();
        assert_uri!(uri, format!("{}/groups/add-users", base_uri()));
    }

    #[test]
    fn admin_groups_more_members_works() {
        let uri = builder().groups().more_members().build();
        assert_uri!(uri, format!("{}/groups/more-members", base_uri()));
    }

    #[test]
    fn admin_groups_add_more_non_members_works() {
        let uri = builder().groups().more_non_members().build();
        assert_uri!(uri, format!("{}/groups/more-non-members", base_uri()));
    }

    #[test]
    fn admin_users_uri_works() {
        let uri = builder().users().build();
        assert_uri!(uri, format!("{}/users", base_uri()));
    }

    #[test]
    fn admin_users_add_group_works() {
        let uri = builder().users().add_group().build();
        assert_uri!(uri, format!("{}/users/add-group", base_uri()));
    }

    #[test]
    fn admin_users_add_groups_works() {
        let uri = builder().users().add_groups().build();
        assert_uri!(uri, format!("{}/users/add-groups", base_uri()));
    }

    #[test]
    fn admin_users_captcha_works() {
        let uri = builder().users().captcha().build();
        assert_uri!(uri, format!("{}/users/captcha", base_uri()));
    }

    #[test]
    fn admin_users_credentials_works() {
        let uri = builder().users().credentials().build();
        assert_uri!(uri, format!("{}/users/credentials", base_uri()));
    }

    #[test]
    fn admin_users_more_members_works() {
        let uri = builder().users().more_members().build();
        assert_uri!(uri, format!("{}/users/more-members", base_uri()));
    }

    #[test]
    fn admin_users_more_non_members_works() {
        let uri = builder().users().more_non_members().build();
        assert_uri!(uri, format!("{}/users/more-non-members", base_uri()));
    }

    #[test]
    fn admin_users_remove_group_works() {
        let uri = builder().users().remove_group().build();
        assert_uri!(uri, format!("{}/users/remove-group", base_uri()));
    }

    #[test]
    fn admin_users_rename_works() {
        let uri = builder().users().rename().build();
        assert_uri!(uri, format!("{}/users/rename", base_uri()));
    }

    #[test]
    fn admin_permissions_works() {
        let uri = builder().permissions().build();
        assert_uri!(uri, format!("{}/permissions", base_uri()));
    }

    #[test]
    fn admin_group_permissions_works() {
        let uri = builder().permissions().groups().build();
        assert_uri!(uri, format!("{}/permissions/groups", base_uri()));
    }

    #[test]
    fn admin_none_group_permissions_works() {
        let uri = builder().permissions().groups().none().build();
        assert_uri!(uri, format!("{}/permissions/groups/none", base_uri()));
    }

    #[test]
    fn admin_user_permissions_works() {
        let uri = builder().permissions().users().build();
        assert_uri!(uri, format!("{}/permissions/users", base_uri()));
    }

    #[test]
    fn admin_none_user_permissions_works() {
        let uri = builder().permissions().users().none().build();
        assert_uri!(uri, format!("{}/permissions/users/none", base_uri()));
    }

    #[test]
    fn admin_mail_server_works() {
        let uri = builder().mail_server().build();
        assert_uri!(uri, format!("{}/mail-server", base_uri()));
    }

    #[test]
    fn admin_mail_server_sender_address_works() {
        let uri = builder().mail_server().sender_address().build();
        assert_uri!(uri, format!("{}/mail-server/sender-address", base_uri()));
    }
}