use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct BitbucketError {
    pub context: Option<String>,
    pub message: String,
    #[serde(rename(deserialize = "exceptionName"))]
    pub exception_name: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct BitbucketErrors {
    pub errors: Vec<BitbucketError>,
}

impl fmt::Display for BitbucketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BitbucketError {}

impl fmt::Display for BitbucketErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        let mut errors = String::new();

        for (i, error) in self.errors.iter().enumerate() {
            errors.push_str(&format!("    {}. {}\n", i + 1, error));
        }

        let msg = format!(
            "The following errors where encountered:\n{errors}",
            errors = errors
        );

        write!(f, "{}", msg)
    }
}

impl std::error::Error for BitbucketErrors {}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Link {
    pub url: String,
    pub rel: String,
}

#[derive(Debug, Deserialize, Default, Eq, PartialEq)]
pub struct LinkPart {
    pub href: String
}

#[derive(Debug, Deserialize, Default, Eq, PartialEq)]
pub struct Links {
    #[serde(rename(deserialize = "self"))]
    pub parts: Vec<LinkPart>
}


#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PagedResponse<T> {
    pub size: u32,
    pub limit: u32,
    #[serde(rename(deserialize = "isLastPage"))]
    pub is_last_page: bool,
    pub values: Vec<T>,
    pub start: u32,
    #[serde(skip)]
    pub filter: u32,
    #[serde(rename(deserialize = "nextPageStart"))]
    pub next_page_start: Option<u32>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Project {
    pub key: String,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub r#type: String,
    pub link: Option<Link>,
    pub links: Links,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RepositoryCloneLinkPart {
    pub href: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RepositoryLinks {
    pub clone: Vec<RepositoryCloneLinkPart>,
    #[serde(rename(deserialize = "self"))]
    pub parts: Links,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Repository {
    pub slug: String,
    pub id: u32,
    pub name: String,
    #[serde(rename(deserialize = "scmId"))]
    pub scm: String,
    pub state: String,
    #[serde(rename(deserialize = "statusMessage"))]
    pub status: String,
    pub forkable: bool,
    pub project: Project,
    pub public: bool,
    #[serde(rename(deserialize = "cloneUrl"))]
    pub clone_url: Option<String>,
    pub link: Option<Link>,
    pub links: RepositoryLinks,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Branch {
    pub id: String,
    #[serde(rename(deserialize = "displayId"))]
    pub display_id: String,
    #[serde(rename(deserialize = "latestChangeset"))]
    pub latest_changeset: String,
    #[serde(rename(deserialize = "latestCommit"))]
    pub latest_commit: String,
    #[serde(rename(deserialize = "isDefault"))]
    pub is_default: bool,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Author {
    pub name: String,
    #[serde(rename(deserialize = "emailAddress"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ParentCommit {
    pub id: String,
    #[serde(rename(deserialize = "displayId"))]
    pub display_id: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Commit {
    pub id: String,
    #[serde(rename(deserialize = "displayId"))]
    pub display_id: String,
    pub author: Author,
    #[serde(rename(deserialize = "authorTimestamp"))]
    pub author_timestamp: u64,
    pub message: String,
    pub parents: Vec<ParentCommit>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct User {
    pub name: String,
    #[serde(rename(deserialize = "emailAddress"))]
    pub email: String,
    pub id: u64,
    #[serde(rename(deserialize = "displayName"))]
    pub display_name: String,
    pub active: bool,
    pub slug: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PullRequestRefRepoProject {
    pub key: String
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PullRequestRefRepo {
    pub slug: String,
    pub name: Option<String>,
    pub project: PullRequestRefRepoProject,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PullRequestRef {
    pub id: String,
    pub repository: PullRequestRefRepo,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PullRequestMember {
    pub user: User,
    pub role: String,
    pub approved: bool,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PullRequest {
    pub id: u64,
    pub version: u32,
    pub title: String,
    pub description: Option<String>,
    // TODO: Make this an enum an deserialize manually
    pub state: String,
    pub open: bool,
    pub closed: bool,
    #[serde(rename(deserialize = "createdDate"))]
    pub date_created: u64,
    #[serde(rename(deserialize = "updatedDate"))]
    pub date_updated: u64,
    #[serde(rename(deserialize = "fromRef"))]
    pub from_ref: PullRequestRef,
    #[serde(rename(deserialize = "toRef"))]
    pub to_ref: PullRequestRef,
    pub locked: bool,
    pub author: PullRequestMember,
    pub reviewers: Vec<PullRequestMember>,
    pub participants: Vec<PullRequestMember>,
    pub link: Option<Link>,
    pub links: Links,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Tag {
    pub id: String,
    #[serde(rename(deserialize = "displayId"))]
    pub display_id: String,
    #[serde(rename(deserialize = "latestChangeset"))]
    pub latest_changeset: String,
    #[serde(rename(deserialize = "latestCommit"))]
    pub latest_commit: String,
    pub hash: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ApplicationProperties {
    pub version: String,
    #[serde(rename(deserialize = "buildNumber"))]
    pub build_number: String,
    #[serde(rename(deserialize = "buildDate"))]
    pub build_date: String,
    #[serde(rename(deserialize = "displayName"))]
    pub name: String,
}

pub enum PullRequestState {
    ALL,
    OPEN,
    MERGED,
    DECLINED,
}

impl PullRequestState {
    pub fn as_str(&self) -> &'static str {
        match self {
            PullRequestState::ALL => "ALL",
            PullRequestState::OPEN => "OPEN",
            PullRequestState::MERGED => "MERGED",
            PullRequestState::DECLINED => "DECLINED",
        }
    }
}