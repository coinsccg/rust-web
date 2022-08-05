#![allow(unused_imports)]

use std::ops::Add;
use std::time::{SystemTime, Duration};
use log::{error, info};
use sqlx::mysql::MySqlPool;
use crate::errors::Error;
use crate::auth::{Claims, encode_jwt};
use crate::dao;

type ServiceResult<T, E=Error> = Result<T, E>;

/// 管理员登录
pub async fn admin_login(pool: &MySqlPool, username: String, password: String) -> ServiceResult<String> {
    dao::find_password(pool, username, password).await?;

    let timestamp = SystemTime::now()
        .add(Duration::from_secs(12000))
        .duration_since(SystemTime::UNIX_EPOCH).unwrap()
        .as_secs();
    let claims = Claims::new(1, timestamp);
    let token = encode_jwt(claims).unwrap();
    Ok(token)
}

/// 激活用户
pub async fn active_user(pool: &MySqlPool, parent: String, owner: String) -> ServiceResult<()> {
    dao::find_active_address(pool, parent, owner).await?;
    Ok(())
}

/// 增加积分
pub async fn add_point(pool: &MySqlPool, owner: String, point: i64) -> ServiceResult<()> {
    dao::add_point(pool, owner, point).await?;
    Ok(())
}

/// 查询余额
pub async fn find_balance(pool: &MySqlPool, owner: String) -> ServiceResult<i64> {
    let point = dao::find_balance(pool, owner).await?;
    Ok(point)
}
