use crate::traits::AsyncRestClient;
use anyhow::Result;
use crate::models::get::{PullRequest, PullRequestState};
use crate::resources::util::accumulate_pages;
use crate::uri_builders::{PullRequestResourceUriBuilder, ResourceUriBuilder, UriBuilder};

pub struct PullRequestResource<'client, C> {
    client: &'client C,
    uri_builder: PullRequestResourceUriBuilder<'client>,
}

impl<'client, C> PullRequestResource<'client, C>
    where
        C: AsyncRestClient
{
    pub fn new(client: &'client C, project: &str, repository: &str) -> Self {
        let uri_builder = ResourceUriBuilder::default()
            .scheme(client.scheme())
            .host(client.host())
            .projects()
            .project(project)
            .repos()
            .repository(repository)
            .pull_requests();

        Self { client, uri_builder }
    }

    pub async fn get_all_pull_requests_with_state(&self, state: PullRequestState) -> Result<Vec<PullRequest>> {
        let uri = format!("{}?state={}", self.uri_builder.build()?, state.as_str());
        accumulate_pages(&uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        }).await
    }

    pub async fn get_all_pull_requests(&self) -> Result<Vec<PullRequest>> {
        self.get_all_pull_requests_with_state(PullRequestState::ALL).await
    }

    pub async fn get_all_open_pull_requests(&self) -> Result<Vec<PullRequest>> {
        self.get_all_pull_requests_with_state(PullRequestState::OPEN).await
    }

    pub async fn get_all_merged_pull_requests(&self) -> Result<Vec<PullRequest>> {
        self.get_all_pull_requests_with_state(PullRequestState::MERGED).await
    }

    pub async fn get_all_declined_pull_requests(&self) -> Result<Vec<PullRequest>> {
        self.get_all_pull_requests_with_state(PullRequestState::DECLINED).await
    }

    pub async fn get_pull_request(&self, id: u64) -> Result<PullRequest> {
        let uri = self.uri_builder.clone().pull_request(id).build()?;
        self.client.get_as(&uri).await
    }
}