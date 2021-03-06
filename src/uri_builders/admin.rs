use crate::uri_builders::{ResourceUriBuilder, UriBuilder, BuildResult, TerminalUriBuilder};
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

    pub fn permissions(self) -> AdminPermissionsUriBuilder<'r> {
        AdminPermissionsUriBuilder::new(self)
    }

    pub fn mail_server(self) -> AdminMailServerUriBuilder<'r> {
        AdminMailServerUriBuilder::new(self)
    }

    #[named]
    pub fn cluster(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn licence(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }
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

    #[named]
    pub fn add_user(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn add_users(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn more_members(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn more_non_members(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn remove_user(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }
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

    #[named]
    pub fn add_group(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn add_groups(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn captcha(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn credentials(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn more_members(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn more_non_members(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn remove_group(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }

    #[named]
    pub fn rename(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }
}

impl<'r> UriBuilder for AdminUserUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/users", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct AdminPermissionsUriBuilder<'r> {
    builder: AdminUriBuilder<'r>,
}

impl<'r> AdminPermissionsUriBuilder<'r> {
    pub fn new(builder: AdminUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn groups(self) -> AdminGroupPermissionUriBuilder<'r> {
        AdminGroupPermissionUriBuilder::new(self)
    }

    pub fn users(self) -> AdminUserPermissionUriBuilder<'r> {
        AdminUserPermissionUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for AdminPermissionsUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/permissions", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct AdminGroupPermissionUriBuilder<'r> {
    builder: AdminPermissionsUriBuilder<'r>,
}

impl<'r> AdminGroupPermissionUriBuilder<'r> {
    pub fn new(builder: AdminPermissionsUriBuilder<'r>) -> Self {
        Self { builder }
    }

    #[named]
    pub fn none(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }
}

impl<'r> UriBuilder for AdminGroupPermissionUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/groups", self.builder.build()?);
        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct AdminUserPermissionUriBuilder<'r> {
    builder: AdminPermissionsUriBuilder<'r>,
}

impl<'r> AdminUserPermissionUriBuilder<'r> {
    pub fn new(builder: AdminPermissionsUriBuilder<'r>) -> Self {
        Self { builder }
    }

    #[named]
    pub fn none(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }
}

impl<'r> UriBuilder for AdminUserPermissionUriBuilder<'r> {
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

    #[named]
    pub fn sender_address(self) -> TerminalUriBuilder<Self> {
        terminal_uri_builder!(self)
    }
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