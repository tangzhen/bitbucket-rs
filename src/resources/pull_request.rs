use crate::models::get::{PullRequest, PullRequestState};
use crate::models::post;
use crate::resources::util::accumulate_pages;
use crate::traits::AsyncRestClient;
use crate::uri_builders::{PullRequestUriBuilder, ResourceUriBuilder, UriBuilder};
use anyhow::Result;

pub struct PullRequestResource<'client, C> {
    client: &'client C,
    uri_builder: PullRequestUriBuilder<'client>,
}

impl<'client, C> PullRequestResource<'client, C>
where
    C: AsyncRestClient,
{
    pub fn new(client: &'client C, project: &'client str, repository: &'client str) -> Self {
        let uri_builder = ResourceUriBuilder::default()
            .scheme(client.scheme())
            .host(client.host())
            .projects()
            .project(project)
            .repos()
            .repository(repository)
            .pull_requests();

        Self {
            client,
            uri_builder,
        }
    }

    pub async fn get_all_pull_requests_with_state(
        &self,
        state: PullRequestState,
    ) -> Result<Vec<PullRequest>> {
        let uri = format!("{}?state={}", self.uri_builder.build()?, state.as_str());
        accumulate_pages(&uri, |uri| {
            let uri = uri.to_owned();
            async move { self.client.get_as(&uri).await }
        })
        .await
    }

    pub async fn get_all_pull_requests(&self) -> Result<Vec<PullRequest>> {
        self.get_all_pull_requests_with_state(PullRequestState::ALL)
            .await
    }

    pub async fn get_all_open_pull_requests(&self) -> Result<Vec<PullRequest>> {
        self.get_all_pull_requests_with_state(PullRequestState::OPEN)
            .await
    }

    pub async fn get_all_merged_pull_requests(&self) -> Result<Vec<PullRequest>> {
        self.get_all_pull_requests_with_state(PullRequestState::MERGED)
            .await
    }

    pub async fn get_all_declined_pull_requests(&self) -> Result<Vec<PullRequest>> {
        self.get_all_pull_requests_with_state(PullRequestState::DECLINED)
            .await
    }

    pub async fn get_pull_request(&self, id: u64) -> Result<PullRequest> {
        let uri = self.uri_builder.clone().pull_request(id).build()?;
        self.client.get_as(&uri).await
    }

    pub async fn create_pull_request(
        &self,
        pull_request: &post::PullRequest,
    ) -> Result<PullRequest> {
        let uri = self.uri_builder.build()?;
        self.client.post(&uri, Some(pull_request)).await
    }
}
