
#[derive(Debug, Clone)]
pub enum Authorization {
    Basic(String, String),
    Bear(String),
}
