use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Login {
    pub user: String,
    pub password: String
}