use std::collections::HashMap;
use actix_web::web::Bytes;
use log::error;
use serde::Deserialize;
use crate::errors::Error;

pub fn parse_body<'a, T>(body: &'a Bytes) -> Result<T, Error>
where T: Deserialize<'a>,
{
    let payload = std::str::from_utf8(body.as_ref()).map_err(|e|{
        error!("parse::parse_body error{:?}", e);
        Error::RequestBadError
    })?;
    let result = serde_json::from_str::<T>(payload).map_err(|e| {
        error!("parse::parse_body error{:?}", e);
        Error::RequestBadError
    })?;
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
                error!("parse::parse_query error({})", Error::ParamMissError.to_string());
                return Err(Error::ParamMissError)
            }
            hash_map.insert(param[0], value);
        }
        Ok(hash_map)
    } else {
        error!("parse::parse_query error({})", Error::ParamAllNotFound.to_string());
        Err(Error::ParamAllNotFound)
    }
}