use std::fmt::Display;
use std::ops::Add;
use std::time::{Duration, SystemTime};
use log::{error, info};
use actix_web::{
    web::{self, Json, Data, Path, Bytes, Query},
    dev::Service,
    Result, route, HttpRequest
};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::{mysql::MySqlPool, query, query_as, Row};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::constant;
use crate::parse::parse_body;
use crate::errors::Error;
use crate::models::{Info, Password, success, fail, Response, Point, ActiveUser};
use crate::auth::bearer_validator;
use crate::services::{login_service, add_point_service, active_user_service};


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

#[derive(Deserialize, Debug)]
pub struct QueryAddress {
    address: String
}

pub async fn find_balance_handler(pool: Data<MySqlPool>, query: Query<QueryAddress>) -> Result<Json<Response<i64>>, Error> {
    let address = &query.address;
    println!("{:?}", address);
    // if address.len() != 42 {
    //     return Err(Error::ParamInvalidError);
    // }

    Ok(Json(success(10)))
}