use crate::uri_builders::{ResourceUriBuilder, UriBuilder, BuildResult, TerminalUriBuilder};
use serde::Serialize;
use serde_plain;

#[derive(Debug)]
pub struct AdminResourceUriBuilder<'r> {
    builder: ResourceUriBuilder<'r>
}

impl<'r> AdminResourceUriBuilder<'r> {
    pub fn new(builder: ResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }

    pub fn groups(self) -> AdminGroupResourceUriBuilder<'r> {
        AdminGroupResourceUriBuilder::new(self)
    }

    pub fn users(self) -> AdminUserResourceUriBuilder<'r> {
        AdminUserResourceUriBuilder::new(self)
    }

    pub fn cluster(self) -> AdminClusterResourceUriBuilder<'r> {
        AdminClusterResourceUriBuilder::new(self)
    }

    pub fn licence(self) -> AdminLicenceResourceUriBuilder<'r> {
        AdminLicenceResourceUriBuilder::new(self)
    }
}

impl<'r> UriBuilder for AdminResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        Ok(format!("{}/admin", self.builder.build()?))
    }
}

#[derive(Debug)]
pub struct AdminClusterResourceUriBuilder<'r> {
    builder: AdminResourceUriBuilder<'r>,
}

impl<'r> AdminClusterResourceUriBuilder<'r> {
    pub fn new(builder: AdminResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }
}

impl<'r> UriBuilder for AdminClusterResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        Ok(format!("{}/cluster", self.builder.build()?))
    }
}

#[derive(Debug)]
pub struct AdminLicenceResourceUriBuilder<'r> {
    builder: AdminResourceUriBuilder<'r>,
}

impl<'r> AdminLicenceResourceUriBuilder<'r> {
    pub fn new(builder: AdminResourceUriBuilder<'r>) -> Self {
        Self { builder }
    }
}

impl<'r> UriBuilder for AdminLicenceResourceUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        Ok(format!("{}/licence", self.builder.build()?))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum AdminGroupAction {
    AddUser,
    AddUsers,
    MoreMembers,
    MoreNonMembers,
    RemoveUser,
}

#[derive(Debug)]
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
        let uri = format!("{}/groups", self.builder.build()?);
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

#[derive(Debug, Serialize)]
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

#[derive(Debug)]
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
        let uri = format!("{}/users", self.builder.build()?);
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

    #[test]
    fn admin_resource_uri_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).admin().build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin");
    }

    #[test]
    fn admin_cluster_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).admin().cluster().build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/cluster");
    }

    #[test]
    fn admin_licence_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).admin().licence().build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/licence");
    }

    #[test]
    fn admin_groups_resource_uri_works() {
        let uri = ResourceUriBuilder::default().host(TEST_HOST).admin().groups().build();
        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/groups");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/groups/add-user");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/groups/add-users");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/groups/more-members");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/groups/more-non-members");
    }

    #[test]
    fn admin_users_resource_uri_works() {
        let uri = ResourceUriBuilder::default()
            .host(TEST_HOST)
            .admin()
            .users()
            .build();

        assert!(uri.is_ok());
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/users");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/users/add-group");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/users/add-groups");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/users/captcha");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/users/credentials");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/users/more-members");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/users/more-non-members");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/users/remove-group");
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
        assert_eq!(uri.unwrap(), "http://stash.test.com/rest/api/1.0/admin/users/rename");
    }
}