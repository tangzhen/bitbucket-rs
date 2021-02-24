use crate::{models::get::Branch, resources::util::*, traits::AsyncRestClient};
use anyhow::Result;

pub struct BranchResource<'client, C> {
    client: &'client C,
    uri: String,
}

impl<'client, C> BranchResource<'client, C>
    where
        C: AsyncRestClient,
{
    pub fn new(client: &'client C, project: &str, repository: &str) -> Self {
        let uri = format!("{}/projects/{}/repos/{}/branches", client.uri(), project, repository);
        Self { client, uri }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub async fn get_all_branches(&self) -> Result<Vec<Branch>> {
        accumulate_pages(&self.uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        }).await
    }

    pub async fn get_default_branch(&self) -> Result<Branch> {
        let uri = format!("{}/default", self.uri);
        self.client.get_as(&uri).await
    }
}
