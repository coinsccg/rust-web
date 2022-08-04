use std::fmt;
use std::fmt::Formatter;
use serde::Serialize;
use actix_http::StatusCode;
use actix_web::{error::ResponseError, body::BoxBody, HttpResponse, http::header};
use actix_web::test::status_service;
use crate::models::{fail, success};


#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("Wrong user name or password")]
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
    #[error("deserialize failed")]
    DeserializeFail,
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
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::UserOrPasswordError => StatusCode::OK,
            Error::ParamInvalidError => StatusCode::OK,
            Error::RequestBadError => StatusCode::OK,
            Error::DeserializeFail => StatusCode::OK,
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