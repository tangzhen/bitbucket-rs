use serde::Serialize;

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct Project {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct User {
    pub name: String,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct PullRequestRefRepoProject {
    pub key: String,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct PullRequestRefRepo {
    pub slug: String,
    pub name: Option<String>,
    pub project: PullRequestRefRepoProject,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct PullRequestRef {
    pub id: String,
    pub repository: PullRequestRefRepo,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct PullRequest {
    pub title: String,
    pub description: Option<String>,
    #[serde(rename(serialize = "fromRef"))]
    pub from_ref: PullRequestRef,
    #[serde(rename(serialize = "toRef"))]
    pub to_ref: PullRequestRef,
    pub close_source_branch: bool,
    pub reviewers: Vec<User>,
}
