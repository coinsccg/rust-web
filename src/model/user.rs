use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Point {
    pub owner: String,
    pub point: i64,
}

#[derive(Deserialize, Debug)]
pub struct ActiveUser {
    pub parent: String,
    pub owner: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryAddress {
    pub owner: String
}


