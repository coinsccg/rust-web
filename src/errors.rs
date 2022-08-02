use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Failed to encode claims")]
    EncodeJWTError(String),
    #[error("Failed to decode claims")]
    DecodeJWTError(String),
    #[error("unknown error")]
    Unknown,
}