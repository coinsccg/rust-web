use std::fmt::Display;
use std::ops::Add;
use std::time::{Duration, SystemTime};
use log::error;
use actix_web::{
    web::{self, Json, Data, Path, Bytes, Query},
    dev::Service,
    Result, route, HttpRequest
};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::{mysql::MySqlPool, query};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::constant;
use crate::parse::{parse_body, parse_query};
use crate::errors::Error;
use crate::models::{
    Password,
    success,
    fail,
    Response,
    Point,
    ActiveUser,
    QueryAddress
};
use crate::services::{
    login_service,
    add_point_service,
    active_user_service,
    find_balance_service
};


/// 获取钱包
pub async fn get_wallet_handler() -> Result<Json<Response<String>>, Error> {
    Ok(Json(success(constant::WALLET.to_string())))
}

/// 后台登录
pub async fn login_handler(pool: Data<MySqlPool>, body: Bytes) -> Result<Json<Response<String>>, Error> {
    let result = parse_body::<Password>(&body)?;
    match login_service(pool.get_ref(), result).await {
        Ok(token) => {
            Ok(Json(success(token)))
        }
        Err(e) => {
            error!("{:?}", e);
            Err(e)
        }
    }
}

/// 激活地址
pub async fn activate_user_handler(pool: Data<MySqlPool>, body: Bytes, head: HttpRequest) -> Result<Json<Response<String>>, Error> {
    println!("{:?}", head.headers().get("authorization"));

    let result = parse_body::<ActiveUser>(&body)?;
    if result.parent.len() != 42 && result.owner.len() != 42 {
        return Err(Error::ParamInvalidError);
    }

    match active_user_service(pool.get_ref(), result.parent, result.owner).await {
        Ok(_) => {
            Ok(Json(success(String::new())))
        }
        Err(e) => {
            error!("{:?}", e);
            Err(e)
        }
    }
}

/// 增加积分
pub async fn add_point_handler(pool: Data<MySqlPool>, body: Bytes) -> Result<Json<Response<String>>, Error> {
    let result = parse_body::<Point>(&body)?;
    if result.owner.len() != 42 && result.point <= 0 {
        return Err(Error::ParamInvalidError);
    }
    match add_point_service(pool.get_ref(), result.owner, result.point).await {
        Ok(_) => {
            Ok(Json(success(String::new())))
        }
        Err(e) => {
            error!("{:?}", e);
            Err(e)
        }
    }
}

pub async fn find_balance_handler(pool: Data<MySqlPool>, query: HttpRequest) -> Result<Json<Response<i64>>, Error> {
    let query_map = parse_query(query.query_string())?;
    let owner = *query_map.get("owner").unwrap();
    if owner.len() != 42 {
        return Err(Error::ParamInvalidError);
    }

    match find_balance_service(pool.as_ref(), owner.to_string()).await {
        Ok(rs) => {
            Ok(Json(success(rs)))
        }
        Err(e) => {
            error!("{:?}", e);
            Err(e)
        }
    }
}