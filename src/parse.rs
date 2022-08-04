use std::collections::HashMap;
use actix_web::web::Bytes;
use serde::Deserialize;
use crate::errors::Error;

pub fn parse_body<'a, T>(body: &'a Bytes) -> Result<T, Error>
where T: Deserialize<'a>,
{
    let payload = std::str::from_utf8(body.as_ref()).map_err(|_|Error::RequestBadError)?;
    let result = serde_json::from_str::<T>(payload).map_err(|_|Error::ParamTypeError)?;
    Ok(result)
}

pub fn parse_query(query: &str) -> Result<HashMap<&str, &str>, Error> {
    let query_vec: Vec<&str> = query.split("&").collect();
    let mut hash_map: HashMap<&str, &str> = HashMap::new();
    if query_vec.len() >= 1 {
        for s in query_vec {
            let param: Vec<&str> = s.split("=").collect();
            let value = *param.get(1).unwrap_or(&"");
            if value.len() == 0 {
                return Err(Error::ParamInvalidError)
            }
            hash_map.insert(param[0], value);
        }
        Ok(hash_map)
    } else {
        Err(Error::ParamInvalidError)
    }
}