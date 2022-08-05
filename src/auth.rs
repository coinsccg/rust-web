use actix_web::{dev::ServiceRequest, http::header::{AUTHORIZATION, HeaderValue}};
use actix_web_httpauth::extractors::{bearer::{BearerAuth, Config}, AuthenticationError};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::errors::Error;
use crate::constant::SECRET;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: i64,
    exp: u64,
}

impl Claims {
    pub fn new(id: i64, exp: u64) -> Self {
        Claims {
            id: id,
            exp: exp,
        }
    }
}

pub(crate) fn encode_jwt(claim: Claims) -> Result<String, Error>{
    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(SECRET.as_ref()))
        .map_err(|_|Error::EncodeJsonWebTokenError);
    token
}

pub(crate) fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let claim = decode::<Claims>(token, &DecodingKey::from_secret(SECRET.as_ref()), &Validation::default())
        .map(|data|data.claims)
        .map_err(|_|Error::DecodeJsonWebTokenError);
    claim
}

pub(crate) async fn bearer_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    if let Ok(claims) = decode_jwt(credentials.token()) {
        let mut req = req;
        req.headers_mut().insert(AUTHORIZATION, HeaderValue::from(claims.id));
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