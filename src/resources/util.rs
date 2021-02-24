use crate::models::get::PagedResponse;
use anyhow::Result;
use std::future::Future;

pub async fn accumulate_pages<F, Fut, R: std::fmt::Debug>(uri: &str, method: F) -> Result<Vec<R>>
    where
        F: Fn(&str) -> Fut,
        Fut: Future<Output=Result<PagedResponse<R>>>,
{
    let limit = 50u32;
    let base_uri = format!("{}?limit={}", uri, limit);
    let mut res = Vec::new();

    let mut uri = base_uri.clone();
    loop {
        let page: PagedResponse<_> = method(&uri).await?;
        res.extend(page.values);

        if page.is_last_page {
            break Ok(res);
        }

        uri = format!("{}&start={}", base_uri, page.next_page_start.unwrap());
    }
}
