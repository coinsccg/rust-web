use std::ops::Add;
use std::time::{SystemTime, Duration};
use log::{error, info};
use sqlx::mysql::MySqlPool;
use sqlx::{query, Row};
use crate::errors::Error;
use crate::auth::{Claims, encode_jwt};
use crate::models::Password;

type ServiceResult<T, E=Error> = Result<T, E>;

/// 登录
pub async fn login_service(pool: &MySqlPool, user: Password) -> ServiceResult<String> {
    let row = query(
        "SELECT id FROM users WHERE username = ? AND password = ?",
    ).bind(user.username).bind(user.password).fetch_one(pool).await;

    match row {
        Ok(rs) => {
            if let Ok(_id) = rs.try_get::<i64, _>("id"){}
        }
        Err(e) => {
            error!("{:?}", e);
            return Err(Error::ParamInvalidError);
        }
    }

    let timestamp = SystemTime::now()
        .add(Duration::from_secs(12000))
        .duration_since(SystemTime::UNIX_EPOCH).unwrap()
        .as_secs();
    let claims = Claims::new(1, timestamp);
    let token = encode_jwt(claims).unwrap();
    Ok(token)
}

/// 激活用户
pub async fn active_user_service(pool: &MySqlPool, parent: String, owner: String) -> ServiceResult<()> {
    let row = query(
        "SELECT id FROM points WHERE parent = ? AND owner = ?",
    )
        .bind(parent.clone())
        .bind(owner.clone())
        .fetch_one(pool)
        .await;


    match row {
        Ok(_) => {
            Err(Error::AddressAlreadyExists)
        }
        Err(sqlx::Error::RowNotFound) => {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let rows = query(
                "INSERT INTO points(parent, owner, point, create_time, update_time) VALUES(?,?,?,?,?)"
            )
                .bind(parent)
                .bind(owner)
                .bind(0)
                .bind(timestamp)
                .bind(timestamp)
                .execute(pool).await;
            match rows {
                Ok(rs) => {
                    if rs.rows_affected() > 0 {

                        Ok(())
                    } else {
                        Err(Error::AddressNotFound)
                    }
                }
                Err(e) =>{
                    error!("{:?}", e);
                    Err(Error::InternalServerError)
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(Error::InternalServerError)
        }
    }
}

/// 增加积分
pub async fn add_point_service(pool: &MySqlPool, owner: String, point: i64) -> ServiceResult<()> {
    let row = query(
        "UPDATE points SET point = point + ? WHERE owner = ?"
    )
        .bind(point)
        .bind(owner)
        .execute(pool)
        .await;

    match row {
        Ok(rs) => {
            if rs.rows_affected() > 0 {
                Ok(())
            } else {
                Err(Error::AddressNotFound)
            }
        }
        Err(e) =>{
            error!("{:?}", e);
            Err(Error::InternalServerError)
        }
    }
}

pub async fn find_balance_service(pool: &MySqlPool, owner: String) -> ServiceResult<i64> {
    let row = query(
        "SELECT point FROM points WHERE owner = ?"
    )
        .bind(owner)
        .fetch_one(pool)
        .await;
    match row {
        Ok(rs) => {
            Ok(rs.get("point"))
        }
        Err(sqlx::Error::RowNotFound) => {
            Err(Error::AddressNotFound)
        }
        Err(e) => {
            error!("{:?}", e);
            Err(Error::InternalServerError)
        }
    }
}
