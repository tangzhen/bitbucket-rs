use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PagedResponse<T> {
    size: u32,
    limit: u32,
    #[serde(rename(deserialize = "isLastPage"))]
    is_last_page: bool,
    values: Vec<T>,
    start: u32,
    #[serde(skip)]
    filter: u32,
    #[serde(default)]
    #[serde(rename(deserialize = "nextPageStart"))]
    next_page_start: u32,
}
