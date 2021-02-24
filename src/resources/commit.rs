use crate::{models::get::Commit, resources::util::*, traits::AsyncRestClient};
use anyhow::Result;

pub struct CommitResource<'client, C> {
    client: &'client C,
    uri: String,
}

impl<'client, C> CommitResource<'client, C>
    where
        C: AsyncRestClient,
{
    pub fn new(client: &'client C, project: &str, repository: &str) -> Self {
        let uri = format!("{}/projects/{}/repos/{}/commits", client.uri(), project, repository);
        Self { client, uri }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub async fn get_all_commits(&self) -> Result<Vec<Commit>> {
        accumulate_pages(&self.uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        })
            .await
    }

    pub async fn get_commit(&self, commit: &str) -> Result<Commit> {
        let uri = format!("{}/{}", self.uri, commit);
        self.client.get_as(&uri).await
    }
}
