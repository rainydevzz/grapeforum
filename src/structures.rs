use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Login {
    pub user: String,
    pub password: String
}

#[derive(Deserialize, Debug)]
pub struct Register {
    pub user: String,
    pub password: String,
    pub confirm_password: String
}

#[derive(Deserialize, Debug)]
pub struct CreatePost {
    pub content: String,
    pub title: String
}

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub comment: String
}

#[derive(Deserialize, Debug)]
pub struct Bio {
    pub bio: String
}

#[derive(Deserialize, Debug)]
pub struct UpdateForm {
    pub content: String
}