use crate::traits::AsyncRestClient;
use anyhow::Result;
use crate::models::get::{PullRequest, PullRequestState};
use crate::resources::util::accumulate_pages;

pub struct PullRequestResource<'client, C> {
    client: &'client C,
    uri: String,
}

impl<'client, C> PullRequestResource<'client, C>
    where
        C: AsyncRestClient
{
    pub fn new(client: &'client C, project: &str, repository: &str) -> Self {
        let uri = format!("{}/projects/{}/repos/{}/pull-requests", "http://stash.test.com/rest/api/1.0", project, repository);
        Self { client, uri }
    }

    pub async fn get_all_pull_requests_with_state(&self, state: PullRequestState) -> Result<Vec<PullRequest>> {
        let uri = format!("{}?state={}", self.uri, state.as_str());
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
        let uri = format!("{}/{}", self.uri, id);
        self.client.get_as(&uri).await
    }
}