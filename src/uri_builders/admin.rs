use crate::uri_builders::{ResourceUriBuilder, UriBuilder, BuildResult, TerminalUriBuilder};
use serde::Serialize;
use serde_plain;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
enum AdminAction {
    Groups,
    Users,
    Cluster,
    Licence,
}

#[derive(Debug, Clone)]
pub struct AdminResourceUriBuilder<'r> {
    builder: ResourceUriBuilder<'r>,
    action: Option<AdminAction>,
}

impl<'r> AdminResourceUriBuilder<'r> {
    pub fn new(builder: ResourceUriBuilder<'r>) -> Self {
        Self { builder, action: None }
    }

    pub fn groups(mut self) -> AdminGroupResourceUriBuilder<'r> {
        self.action = Some(AdminAction::Groups);
        AdminGroupResourceUriBuilder::new(self)
    }

    pub fn users(mut self) -> AdminUserResourceUriBuilder<'r> {
        self.action = Some(AdminAction::Users);
        AdminUserResourceUriBuilder::new(self)
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

impl<'r> UriBuilder for AdminResourceUriBuilder<'r> {
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
pub struct AdminGroupResourceUriBuilder<'r> {
    builder: AdminResourceUriBuilder<'r>,
    action: Option<AdminGroupAction>,
}

impl<'r> AdminGroupResourceUriBuilder<'r> {
    pub fn new(builder: AdminResourceUriBuilder<'r>) -> Self {
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

impl<'r> UriBuilder for AdminGroupResourceUriBuilder<'r> {
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
pub struct AdminUserResourceUriBuilder<'r> {
    builder: AdminResourceUriBuilder<'r>,
    action: Option<AdminUserAction>,
}

impl<'r> AdminUserResourceUriBuilder<'r> {
    pub fn new(builder: AdminResourceUriBuilder<'r>) -> Self {
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

impl<'r> UriBuilder for AdminUserResourceUriBuilder<'r> {
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
    use crate::uri_builders::tests::TEST_HOST;

    fn base_uri() -> String {
        format!("{}/admin", crate::uri_builders::tests::base_uri())
    }

    #[test]
    fn admin_resource_uri_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).admin().build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), base_uri());
    }

    #[test]
    fn admin_cluster_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).admin().cluster().build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/cluster", base_uri()));
    }

    #[test]
    fn admin_licence_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).admin().licence().build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/licence", base_uri()));
    }

    #[test]
    fn admin_groups_resource_uri_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).admin().groups().build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/groups", base_uri()));
    }

    #[test]
    fn admin_groups_add_user_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .groups()
            .add_user()
            .build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/groups/add-user", base_uri()));
    }

    #[test]
    fn admin_groups_add_users_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .groups()
            .add_users()
            .build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/groups/add-users", base_uri()));
    }

    #[test]
    fn admin_groups_more_members_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .groups()
            .more_members()
            .build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/groups/more-members", base_uri()));
    }

    #[test]
    fn admin_groups_add_more_non_members_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .groups()
            .more_non_members()
            .build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/groups/more-non-members", base_uri()));
    }

    #[test]
    fn admin_users_resource_uri_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/users", base_uri()));
    }

    #[test]
    fn admin_users_add_group_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .add_group()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/users/add-group", base_uri()));
    }

    #[test]
    fn admin_users_add_groups_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .add_groups()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/users/add-groups", base_uri()));
    }

    #[test]
    fn admin_users_captcha_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .captcha()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/users/captcha", base_uri()));
    }

    #[test]
    fn admin_users_credentials_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .credentials()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/users/credentials", base_uri()));
    }

    #[test]
    fn admin_users_more_members_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .more_members()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/users/more-members", base_uri()));
    }

    #[test]
    fn admin_users_more_non_members_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .more_non_members()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/users/more-non-members", base_uri()));
    }

    #[test]
    fn admin_users_remove_group_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .remove_group()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/users/remove-group", base_uri()));
    }

    #[test]
    fn admin_users_rename_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .rename()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), format!("{}/users/rename", base_uri()));
    }
}