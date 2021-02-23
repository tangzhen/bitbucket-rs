use crate::{models::{Repository, Tag}, resources::util::*, traits::AsyncRestClient};
use anyhow::Result;

pub struct RepositoryResource<'client, C> {
    client: &'client C,
    uri: String,
}

impl<'client, C> RepositoryResource<'client, C>
    where
        C: AsyncRestClient,
{
    pub fn new(client: &'client C, project: &str) -> Self {
        let uri = format!("{}/projects/{}/repos", client.uri(), project);
        Self { client, uri }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub async fn get_all_repositories(&self) -> Result<Vec<Repository>> {
        accumulate_pages(&self.uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        }).await
    }

    pub async fn get_repository(&self, repository: &str) -> Result<Repository> {
        let uri = format!("{}/{}", self.uri, repository);
        self.client.get_as(&uri).await
    }

    pub async fn get_all_repository_tags(&self, repository: &str) -> Result<Vec<Tag>> {
        let uri = format!("{}/{}/tags", self.uri, repository);
        accumulate_pages(&uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        }).await
    }
}