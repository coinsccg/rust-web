use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid user name or password")]
    ParamInvalidError,
    #[error("User does not exist")]
    InternalServerError,
    #[error("Server internal error")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Request data invalid")]
    RequestBadError,
    #[error("Database connection failed")]
    DatabaseConnect,
    #[error("Failed to encode claims")]
    EncodeJWTError,
    #[error("Failed to decode claims")]
    DecodeJWTError,
    #[error("Unknown error")]
    Unknown,
}

impl Error {
    pub fn to_string(self) -> String {
        format!("{}", self)
    }
}