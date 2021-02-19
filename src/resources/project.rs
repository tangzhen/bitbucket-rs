use anyhow::Result;

use crate::{
    client::RestClient,
    models::{
        paged_response::PagedResponse,
        project::{PagedProjects, Project},
    },
};

pub struct ProjectResource<'client, C>
where
    C: RestClient,
{
    client: &'client C,
    uri: String,
}

impl<'client, C> ProjectResource<'client, C>
where
    C: RestClient,
{
    pub fn new(client: &'client C) -> Self {
        let uri = format!("{}/projects", client.uri());
        Self { client, uri }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub async fn get_all_projects(&self) -> Result<PagedProjects> {
        self.client.get(&self.uri).await
    }

    pub async fn get_project(&self, project: &str) -> Result<Project> {
        let uri = format!("{}/{}", self.uri, project);
        self.client.get(&uri).await
    }
}

mod tests {
    use crate::{auth::Authorization, client::BitbucketClient};

    use super::*;

    fn client() -> BitbucketClient {
        BitbucketClient::new("stash.test.com")
    }

    fn auth_client() -> BitbucketClient {
        BitbucketClient::with_auth("stash.test.com", Authorization::new("george", "test"))
    }

    #[test]
    fn uri_works() {
        let client = client();
        let resource = ProjectResource::new(&client);
        assert_eq!(resource.uri(), "/projects");
    }
}
