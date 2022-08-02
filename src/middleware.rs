use actix_web::{web, dev::ServiceRequest, Error, FromRequest};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError
};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::errors::CustomError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: u32,
    exp: u64,
}

impl Claims {
    pub fn new(id: u32, exp: u64) -> Self {
        Claims {
            id: id,
            exp: exp,
        }
    }
}

pub(crate) fn encode_jwt(claim: Claims) -> Result<String, CustomError>{
    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret("secret".as_ref()))
        .map_err(|e|CustomError::EncodeJWTError(e.to_string()));
    token
}

pub(crate) fn decode_jwt(token: &str) -> Result<Claims, CustomError> {
    let claim = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())
        .map(|data|data.claims)
        .map_err(|e|CustomError::DecodeJWTError(e.to_string()));
    println!("{:?}", claim);
    claim
}

pub(crate) async fn bearer_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if let Ok(claims) = decode_jwt(credentials.token()) {
        Ok(req)
    } else {
        let config = req
            .app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default)
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        Err((AuthenticationError::from(config).into(), req))
    }
}