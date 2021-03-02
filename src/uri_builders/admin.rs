use crate::uri_builders::{ResourceUriBuilder, UriBuilder, BuildResult, TerminalUriBuilder};
use serde::Serialize;
use serde_plain;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
enum AdminAction {
    Groups,
    Users,
    Permissions,
    Cluster,
    Licence,
    MailServer,
}

#[derive(Debug, Clone)]
pub struct AdminUriBuilder<'r> {
    builder: ResourceUriBuilder<'r>,
    action: Option<AdminAction>,
}

impl<'r> AdminUriBuilder<'r> {
    pub fn new(builder: ResourceUriBuilder<'r>) -> Self {
        Self { builder, action: None }
    }

    pub fn groups(mut self) -> AdminGroupUriBuilder<'r> {
        self.action = Some(AdminAction::Groups);
        AdminGroupUriBuilder::new(self)
    }

    pub fn users(mut self) -> AdminUserUriBuilder<'r> {
        self.action = Some(AdminAction::Users);
        AdminUserUriBuilder::new(self)
    }

    pub fn permissions(mut self) -> AdminPermissionsUriBuilder<'r> {
        self.action = Some(AdminAction::Permissions);
        AdminPermissionsUriBuilder::new(self)
    }

    pub fn mail_server(mut self) -> AdminMailServerUriBuilder<'r> {
        self.action = Some(AdminAction::MailServer);
        AdminMailServerUriBuilder::new(self)
    }

    pub fn cluster(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminAction::Cluster);
        TerminalUriBuilder::new(self)
    }

    pub fn licence(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminAction::Licence);
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for AdminUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/admin", self.builder.build()?);
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

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
enum AdminGroupAction {
    AddUser,
    AddUsers,
    MoreMembers,
    MoreNonMembers,
    RemoveUser,
}

#[derive(Debug, Clone)]
pub struct AdminGroupUriBuilder<'r> {
    builder: AdminUriBuilder<'r>,
    action: Option<AdminGroupAction>,
}

impl<'r> AdminGroupUriBuilder<'r> {
    pub fn new(builder: AdminUriBuilder<'r>) -> Self {
        Self { builder, action: None }
    }

    pub fn add_user(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminGroupAction::AddUser);
        TerminalUriBuilder::new(self)
    }

    pub fn add_users(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminGroupAction::AddUsers);
        TerminalUriBuilder::new(self)
    }

    pub fn more_members(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminGroupAction::MoreMembers);
        TerminalUriBuilder::new(self)
    }

    pub fn more_non_members(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminGroupAction::MoreNonMembers);
        TerminalUriBuilder::new(self)
    }

    pub fn remove_user(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminGroupAction::RemoveUser);
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for AdminGroupUriBuilder<'r> {
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

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
enum AdminUserAction {
    AddGroup,
    AddGroups,
    Captcha,
    Credentials,
    MoreMembers,
    MoreNonMembers,
    RemoveGroup,
    Rename,
}

#[derive(Debug, Clone)]
pub struct AdminUserUriBuilder<'r> {
    builder: AdminUriBuilder<'r>,
    action: Option<AdminUserAction>,
}

impl<'r> AdminUserUriBuilder<'r> {
    pub fn new(builder: AdminUriBuilder<'r>) -> Self {
        Self { builder, action: None }
    }

    pub fn add_group(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminUserAction::AddGroup);
        TerminalUriBuilder::new(self)
    }

    pub fn add_groups(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminUserAction::AddGroups);
        TerminalUriBuilder::new(self)
    }

    pub fn captcha(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminUserAction::Captcha);
        TerminalUriBuilder::new(self)
    }

    pub fn credentials(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminUserAction::Credentials);
        TerminalUriBuilder::new(self)
    }

    pub fn more_members(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminUserAction::MoreMembers);
        TerminalUriBuilder::new(self)
    }

    pub fn more_non_members(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminUserAction::MoreNonMembers);
        TerminalUriBuilder::new(self)
    }

    pub fn remove_group(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminUserAction::RemoveGroup);
        TerminalUriBuilder::new(self)
    }

    pub fn rename(mut self) -> TerminalUriBuilder<Self> {
        self.action = Some(AdminUserAction::Rename);
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for AdminUserUriBuilder<'r> {
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

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
enum AdminPermissionAction {
    Groups,
    Users,
}

#[derive(Debug, Clone)]
pub struct AdminPermissionsUriBuilder<'r> {
    builder: AdminUriBuilder<'r>,
    action: Option<AdminPermissionAction>,
}

impl<'r> AdminPermissionsUriBuilder<'r> {
    pub fn new(builder: AdminUriBuilder<'r>) -> Self {
        Self { builder, action: None }
    }

    pub fn groups(mut self) -> AdminGroupPermissionUriBuilder<'r> {
        self.action = Some(AdminPermissionAction::Groups);
        AdminGroupPermissionUriBuilder::new(self)
    }

    pub fn users(mut self) -> AdminUserPermissionUriBuilder<'r> {
        self.action = Some(AdminPermissionAction::Users);
        AdminUserPermissionUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for AdminPermissionsUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = self.builder.build()?;
        let uri = match &self.action {
            None => uri,
            Some(action) => {
                let action = serde_plain::to_string(action).unwrap();
                format!("{}/{}", uri, action)
            }
        };

        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct AdminGroupPermissionUriBuilder<'r> {
    builder: AdminPermissionsUriBuilder<'r>,
    none: bool,
}

impl<'r> AdminGroupPermissionUriBuilder<'r> {
    pub fn new(builder: AdminPermissionsUriBuilder<'r>) -> Self {
        Self { builder, none: false }
    }

    pub fn none(mut self) -> TerminalUriBuilder<Self> {
        self.none = true;
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for AdminGroupPermissionUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = self.builder.build()?;
        let uri = if self.none {
            format!("{}/none", uri)
        } else {
            uri
        };

        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct AdminUserPermissionUriBuilder<'r> {
    builder: AdminPermissionsUriBuilder<'r>,
    none: bool,
}

impl<'r> AdminUserPermissionUriBuilder<'r> {
    pub fn new(builder: AdminPermissionsUriBuilder<'r>) -> Self {
        Self { builder, none: false }
    }

    pub fn none(mut self) -> TerminalUriBuilder<Self> {
        self.none = true;
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for AdminUserPermissionUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = self.builder.build()?;
        let uri = if self.none {
            format!("{}/none", uri)
        } else {
            uri
        };

        Ok(uri)
    }
}

#[derive(Debug, Clone)]
pub struct AdminMailServerUriBuilder<'r> {
    builder: AdminUriBuilder<'r>,
    sender_address: bool,
}

impl<'r> AdminMailServerUriBuilder<'r> {
    pub fn new(builder: AdminUriBuilder<'r>) -> Self {
        Self { builder, sender_address: false }
    }

    pub fn sender_address(mut self) -> TerminalUriBuilder<Self> {
        self.sender_address = true;
        TerminalUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for AdminMailServerUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = self.builder.build()?;
        let uri = if self.sender_address {
            format!("{}/sender-address", uri)
        } else {
            uri
        };

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