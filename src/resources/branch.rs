use crate::{models::get::Branch, resources::util::*, traits::AsyncRestClient};
use anyhow::Result;
use crate::uri_builders::{BranchResourceUriBuilder, ResourceUriBuilder, UriBuilder};

pub struct BranchResource<'client, C> {
    client: &'client C,
    uri_builder: BranchResourceUriBuilder<'client>,
}

impl<'client, C> BranchResource<'client, C>
    where
        C: AsyncRestClient,
{
    pub fn new(client: &'client C, project: &'client str, repository: &'client str) -> Self {
        let uri_builder = ResourceUriBuilder::default()
            .scheme(client.scheme())
            .host(client.host())
            .projects().project(project)
            .repos().repository(repository)
            .branches();

        Self { client, uri_builder }
    }

    pub async fn get_all_branches(&self) -> Result<Vec<Branch>> {
        let uri = self.uri_builder.build()?;
        accumulate_pages(&uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        }).await
    }

    pub async fn get_default_branch(&self) -> Result<Branch> {
        let uri = self.uri_builder.clone().default().build()?;
        self.client.get_as(&uri).await
    }
}
