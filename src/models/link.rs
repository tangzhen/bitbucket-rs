use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Link {
    url: String,
    rel: String,
}

#[derive(Debug, Deserialize)]
pub struct LinkPart {
    href: String,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    #[serde(rename(deserialize = "self"))]
    parts: Vec<LinkPart>,
}
