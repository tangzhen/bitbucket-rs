use serde::Serialize;

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct Project {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}