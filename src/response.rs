use serde::Serialize;

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