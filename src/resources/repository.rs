use crate::{models::get::{Repository, Tag}, resources::util::*, traits::AsyncRestClient};
use anyhow::Result;
use crate::uri_builders::{RepositoryResourceUriBuilder, ResourceUriBuilder, UriBuilder};

pub struct RepositoryResource<'client, C> {
    client: &'client C,
    uri_builder: RepositoryResourceUriBuilder<'client>,
}

impl<'client, C> RepositoryResource<'client, C>
    where
        C: AsyncRestClient,
{
    pub fn new(client: &'client C, project: &'client str) -> Self {
        let uri_builder = ResourceUriBuilder::default()
            .scheme(client.scheme())
            .host(client.host())
            .projects().project(project)
            .repos();

        Self { client, uri_builder }
    }

    pub async fn get_all_repositories(&self) -> Result<Vec<Repository>> {
        let uri = self.uri_builder.build()?;
        accumulate_pages(&uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        }).await
    }

    pub async fn get_repository(&self, repository: &str) -> Result<Repository> {
        let uri = self.uri_builder.clone().repository(repository).build()?;
        self.client.get_as(&uri).await
    }

    pub async fn get_all_repository_tags(&self, repository: &str) -> Result<Vec<Tag>> {
        let uri = self.uri_builder.clone().repository(repository).tags().build()?;
        accumulate_pages(&uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        }).await
    }
}