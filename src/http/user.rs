#![allow(unused_imports)]

use log::error;
use actix_web::{
    web::{self, Json, Data, Bytes},
    Result, HttpRequest
};
use sqlx::mysql::MySqlPool;
use crate::constant;
use crate::http::parse::{parse_body, parse_query};
use crate::errors::Error;
use crate::model::{
    Login,
    Point,
    ActiveUser
};
use crate::response::{Response, success, fail};
use crate::service;


type HttpResult<T, E=Error> = Result<Json<Response<T>>, E>;

/// 查询钱包地址
pub async fn get_wallet() -> HttpResult<String> {
    Ok(Json(success(constant::WALLET.to_string())))
}

/// 管理员登录
pub async fn admin_login(pool: Data<MySqlPool>, body: Bytes) -> HttpResult<String> {
    let login_json = parse_body::<Login>(&body)?;
    match service::admin_login(pool.get_ref(), login_json.username, login_json.password).await {
        Ok(token) => {
            Ok(Json(success(token)))
        }
        Err(e) => {
            error!("http::user::admin_login error({:?})", e);
            Err(e)
        }
    }
}

/// 激活地址
pub async fn activate_user(pool: Data<MySqlPool>, body: Bytes, head: HttpRequest) -> HttpResult<String> {
    println!("{:?}", head.headers().get("authorization"));

    let active_user_json = parse_body::<ActiveUser>(&body)?;
    if active_user_json.parent.len() != 42 || active_user_json.owner.len() != 42 {
        error!("http::user::activate_user error({})", Error::ParamInvalidError.to_string());
        return Err(Error::ParamInvalidError);
    }

    match service::active_user(pool.get_ref(), active_user_json.parent, active_user_json.owner).await {
        Ok(_) => {
            Ok(Json(success(String::new())))
        }
        Err(e) => {
            error!("http::user::activate_user error({})", e.to_string());
            Err(e)
        }
    }
}

/// 增加积分
pub async fn add_point(pool: Data<MySqlPool>, body: Bytes) -> HttpResult<String> {
    let point_json = parse_body::<Point>(&body)?;
    if point_json.owner.len() != 42 && point_json.point <= 0 {
        error!("http::user::add_point error({:?})", Error::ParamInvalidError.to_string());
        return Err(Error::ParamInvalidError);
    }
    match service::add_point(pool.get_ref(), point_json.owner, point_json.point).await {
        Ok(_) => {
            Ok(Json(success(String::new())))
        }
        Err(e) => {
            error!("http::user::add_point error({:?})", e);
            Err(e)
        }
    }
}

/// 查询积分
pub async fn find_balance(pool: Data<MySqlPool>, query: HttpRequest) -> HttpResult<i64> {
    let query_map = parse_query(query.query_string())?;
    let owner = *query_map.get("owner").unwrap();
    if owner.len() != 42 {
        error!("http::user::find_balance error({:?})", Error::ParamInvalidError.to_string());
        return Err(Error::ParamInvalidError);
    }

    match service::find_balance(pool.as_ref(), owner.to_string()).await {
        Ok(rs) => {
            Ok(Json(success(rs)))
        }
        Err(e) => {
            error!("http::user::find_balance error({:?})", e);
            Err(e)
        }
    }
}