use anyhow::Result;
use bytes::Bytes;

use crate::{models::Project, resources::util::accumulate_pages, traits::AsyncRestClient};

pub struct ProjectResource<'client, C> {
    client: &'client C,
    uri: String,
}

impl<'client, C> ProjectResource<'client, C>
    where
        C: AsyncRestClient,
{
    pub fn new(client: &'client C) -> Self {
        let uri = format!("{}/projects", client.uri());
        Self { client, uri }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>> {
        accumulate_pages(&self.uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        }).await
    }

    pub async fn get_project(&self, project: &str) -> Result<Project> {
        let uri = self.uri_with_project(project);
        self.client.get_as(&uri).await
    }

    pub async fn get_project_avatar(&self, project: &str) -> Result<Bytes> {
        let uri = format!("{}/avatar.png", self.uri_with_project(project));
        let bytes: Bytes = self.client.get(&uri).await?.bytes().await?;
        Ok(bytes)
    }

    #[inline]
    fn uri_with_project(&self, project: &str) -> String {
        format!("{}/{}", self.uri, project)
    }
}
