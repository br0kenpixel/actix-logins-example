use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginEntry {
    pub username: String,
    pub password: String,
}
