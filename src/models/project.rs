use super::{
    link::{Link, Links},
    paged_response::PagedResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    key: String,
    id: u32,
    name: String,
    description: String,
    public: bool,
    r#type: String,
    link: Link,
    links: Links,
}

pub type PagedProjects = PagedResponse<Project>;
