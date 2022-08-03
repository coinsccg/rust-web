use actix_web::{dev::ServiceRequest, web::Bytes};
use actix_http::{Payload, h1};
use actix_web_httpauth::extractors::{bearer::{BearerAuth, Config}, AuthenticationError};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::errors::Error;
use crate::constant::SECRET;

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

pub(crate) fn encode_jwt(claim: Claims) -> Result<String, Error>{
    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(SECRET.as_ref()))
        .map_err(|_|Error::EncodeJWTError);
    token
}

pub(crate) fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let claim = decode::<Claims>(token, &DecodingKey::from_secret(SECRET.as_ref()), &Validation::default())
        .map(|data|data.claims)
        .map_err(|_|Error::DecodeJWTError);
    claim
}

pub(crate) async fn bearer_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    if let Ok(_) = decode_jwt(credentials.token()) {
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