#[macro_use]
extern crate derive_builder;
extern crate async_trait;

pub mod auth;
pub mod client;
pub mod resources;
pub mod traits;
pub mod models;
pub mod uri_builders;

#[derive(Debug, Clone)]
pub enum Scheme {
    HTTP,
    HTTPS,
}
