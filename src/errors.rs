
#[derive(Debug)]
pub enum Error {
    LoginError,
    UserNotFound,
    UserAlreadyExists,
    DatabaseError,
    RequestBadError,
    DatabaseConnect,
    EncodeJWTError,
    DecodeJWTError,
    Unknown,
}

impl Error {
    pub fn to_string(self) -> String {
        match self {
            Error::LoginError => "Wrong user name or password".to_string(),
            Error::UserNotFound => "User does not exist".to_string(),
            Error::DatabaseError => "Database error".to_string(),
            Error::UserAlreadyExists => "User already exists".to_string(),
            Error::RequestBadError => "Request data invalid".to_string(),
            Error::DatabaseConnect => "Database connection failed".to_string(),
            Error::EncodeJWTError => "Failed to encode claims".to_string(),
            Error::DecodeJWTError => "Failed to decode claims".to_string(),
            _ => "unknown error".to_string(),
        }
    }
}