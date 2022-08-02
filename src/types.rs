use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Password {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: T
}

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Response {
            code: 0,
            msg: "success".to_string(),
            data: data,
        }
    }

    pub fn fail(msg: String) -> Self {
        Response {
            code: 1,
            msg: msg,
            data: "".to_string(),
        }
    }
}


#[derive(Deserialize)]
pub struct Info {
    pub user_id: u32,
    pub friend: String,
}