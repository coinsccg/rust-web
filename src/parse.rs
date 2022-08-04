use actix_web::web::Bytes;
use serde::Deserialize;
use crate::errors::Error;

pub fn parse_body<'a, T>(body: &'a Bytes) -> Result<T, Error>
where T: Deserialize<'a>,
{
    let payload = std::str::from_utf8(body.as_ref()).map_err(|_|Error::RequestBadError)?;
    let result = serde_json::from_str::<T>(payload).map_err(|_|Error::DeserializeFail)?;
    Ok(result)
}