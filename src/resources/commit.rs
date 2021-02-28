use crate::{models::get::Commit, resources::util::*, traits::AsyncRestClient};
use anyhow::Result;
use crate::uri_builders::{CommitResourceUriBuilder, ResourceUriBuilder, UriBuilder};

pub struct CommitResource<'client, C> {
    client: &'client C,
    uri_builder: CommitResourceUriBuilder<'client>,
}

impl<'client, C> CommitResource<'client, C>
    where
        C: AsyncRestClient,
{
    pub fn new(client: &'client C, project: &'client str, repository: &'client str) -> Self {
        let uri_builder = ResourceUriBuilder::default()
            .scheme(client.scheme())
            .host(client.host())
            .projects().project(project)
            .repos().repository(repository)
            .commits();

        Self { client, uri_builder }
    }

    pub async fn get_all_commits(&self) -> Result<Vec<Commit>> {
        let uri = self.uri_builder.build()?;
        accumulate_pages(&uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        }).await
    }

    pub async fn get_commit(&self, commit: &str) -> Result<Commit> {
        let uri = self.uri_builder.clone().commit(commit).build()?;
        self.client.get_as(&uri).await
    }
}
