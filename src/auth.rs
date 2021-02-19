#[derive(Debug, Builder, Clone, Eq, PartialEq)]
pub struct Authorization {
    username: String,
    password: String,
}

impl Authorization {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
