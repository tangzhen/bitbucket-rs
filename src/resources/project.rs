use anyhow::Result;
use bytes::Bytes;

use crate::uri_builders::{ProjectUriBuilder, ResourceUriBuilder, UriBuilder};
use crate::{
    models::{get, post},
    resources::util::accumulate_pages,
    traits::AsyncRestClient,
};

pub struct ProjectResource<'client, C> {
    client: &'client C,
    uri_builder: ProjectUriBuilder<'client>,
}

impl<'client, C> ProjectResource<'client, C>
where
    C: AsyncRestClient,
{
    pub fn new(client: &'client C) -> Self {
        let uri_builder = ResourceUriBuilder::default()
            .scheme(client.scheme())
            .host(client.host())
            .projects();

        Self {
            client,
            uri_builder,
        }
    }

    pub async fn get_all_projects(&self) -> Result<Vec<get::Project>> {
        let uri = self.uri_builder.build()?;
        accumulate_pages(&uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        })
        .await
    }

    pub async fn get_project(&self, project: &str) -> Result<get::Project> {
        let uri = self.uri_builder.clone().project(project).build()?;
        self.client.get_as(&uri).await
    }

    pub async fn get_project_avatar(&self, project: &str) -> Result<Bytes> {
        let uri = self.uri_builder.clone().project(project).avatar().build()?;
        let bytes: Bytes = self.client.get(&uri).await?.bytes().await?;
        Ok(bytes)
    }

    pub async fn create_project(&self, project: &post::Project) -> Result<get::Project> {
        let uri = self.uri_builder.build()?;
        self.client.post(&uri, Some(project)).await
    }

    pub async fn update_project(
        &self,
        project: &str,
        payload: &post::Project,
    ) -> Result<get::Project> {
        let uri = self.uri_builder.clone().project(project).build()?;
        self.client.put(&uri, Some(payload)).await
    }

    pub async fn delete_project(&self, project: &str) -> Result<()> {
        let uri = self.uri_builder.clone().project(project).build()?;
        self.client.delete(&uri).await
    }
}
