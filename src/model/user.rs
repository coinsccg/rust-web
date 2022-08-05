use serde::{Deserialize, Serialize};

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


#[derive(Serialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: T
}


pub fn success<T>(data: T) -> Response<T> {
    Response {
        code: 0,
        msg: "success".to_string(),
        data: data,
    }
}

pub fn fail(msg: String) -> Response<String> {
    Response {
        code: 1,
        msg: msg,
        data: "".to_string(),
    }
}


