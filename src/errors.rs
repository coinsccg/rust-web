use std::fmt;
use std::fmt::Formatter;
use serde::Serialize;
use actix_http::StatusCode;
use actix_web::{error::ResponseError, body::BoxBody, HttpResponse, http::header};
use actix_web::test::status_service;
use crate::models::{fail, success};


#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("Error user name or password")]
    UserOrPasswordError,
    #[error("Param invalid")]
    ParamInvalidError,
    #[error("Server internal error")]
    InternalServerError,
    #[error("Address does not exist")]
    AddressNotFound,
    #[error("Address already exists")]
    AddressAlreadyExists,
    #[error("Request data invalid")]
    RequestBadError,
    #[error("Param type invalid")]
    ParamTypeError,
    #[error("Database connection failed")]
    DatabaseConnect,
    #[error("Failed to encode claims")]
    EncodeJsonWebTokenError,
    #[error("Failed to decode claims")]
    DecodeJsonWebTokenError,
    #[error("Unknown error")]
    Unknown,
}

impl Error {
    pub fn to_string(self) -> String {
        format!("{}", self)
    }
    pub fn to_response(self) -> &'static str {
        match self {
            Error::UserOrPasswordError => "用户名或密码错误",
            Error::ParamInvalidError => "参数不合法",
            Error::InternalServerError => "服务器内部错误",
            Error::AddressNotFound => "该地址不存在",
            Error::AddressAlreadyExists => "该地址已经存在",
            Error::RequestBadError => "请求错误",
            Error::ParamTypeError => "请求参数类型错误",
            Error::DatabaseConnect => "数据库连接错误",
            Error::EncodeJsonWebTokenError => "jwt生成失败",
            Error::DecodeJsonWebTokenError => "jwt验证失败",
            Error::Unknown => "不知名错误"
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::UserOrPasswordError => StatusCode::OK,
            Error::ParamInvalidError => StatusCode::OK,
            Error::RequestBadError => StatusCode::OK,
            Error::ParamTypeError => StatusCode::OK,
            Error::DecodeJsonWebTokenError => StatusCode::UNAUTHORIZED,
            Error::AddressNotFound => StatusCode::OK,
            Error::AddressAlreadyExists => StatusCode::OK,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(fail(self.to_string()))
    }
}