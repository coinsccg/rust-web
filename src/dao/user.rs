use std::time::SystemTime;
use sqlx::{mysql::MySqlPool, query, Row};
use log::error;
use crate::errors::Error;

const _SELECT_PASSWORD: &str = "SELECT id FROM users WHERE username = ? AND password = ?";
const _SELECT_ACTIVATE_ADDRESS: &str = "SELECT id FROM points WHERE parent = ? AND owner = ?";
const _INSERT_ADDRESS_POINT: &str = "INSERT INTO points(parent, owner, point, create_time, update_time) VALUES(?,?,?,?,?)";
const _UPDATE_ADDRESS_POINT: &str = "UPDATE points SET point = point + ? WHERE owner = ?";
const _SELECT_ADDRESS_POINT: &str = "SELECT point FROM points WHERE owner = ?";


type DaoResult<T, E=Error> = Result<T, E>;

/// 查询用户
pub async fn find_password(pool: &MySqlPool, username: String, password: String) -> DaoResult<()> {
    let row = query(_SELECT_PASSWORD)
        .bind(username)
        .bind(password)
        .fetch_one(pool)
        .await;

    match row {
        Ok(_) => {
            Ok(())
        }
        Err(_) => {
            error!("dao::user::find_password error({})", Error::UserOrPasswordError.to_string());
            Err(Error::UserOrPasswordError)
        }
    }
}

/// 查询地址是否存在，不存在则插入地址
pub async fn find_active_address(pool: &MySqlPool, parent: String, owner: String) -> DaoResult<()> {
    let row = query(_SELECT_ACTIVATE_ADDRESS)
        .bind(parent.clone())
        .bind(owner.clone())
        .fetch_one(pool)
        .await;

    match row {
        Ok(_) => {
            error!("dao::user::find_active_address error({})", Error::AddressAlreadyActivated.to_string());
            Err(Error::AddressAlreadyActivated)
        }
        Err(sqlx::Error::RowNotFound) => {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let res = query(_INSERT_ADDRESS_POINT)
                .bind(parent)
                .bind(owner)
                .bind(0)
                .bind(timestamp)
                .bind(timestamp)
                .execute(pool).await;
            match res {
                Ok(rs) => {
                    if rs.rows_affected() > 0 {
                        Ok(())
                    } else {
                        error!("dao::user::find_active_address error({})", Error::InternalServerError.to_string());
                        Err(Error::InternalServerError)
                    }
                }
                Err(e) =>{
                    error!("dao::user::find_active_address error({:?})", e);
                    Err(Error::InternalServerError)
                }
            }
        }
        Err(e) => {
            error!("dao::user::find_active_address error({:?})", e);
            Err(Error::InternalServerError)
        }
    }
}

/// 更新地址积分
pub async fn add_point(pool: &MySqlPool, owner: String, point: i64) -> DaoResult<()> {
    let res = query(_UPDATE_ADDRESS_POINT)
        .bind(point)
        .bind(owner)
        .execute(pool)
        .await;

    match res  {
        Ok(rs) => {
            if rs.rows_affected() > 0 {
                Ok(())
            } else {
                error!("dao::user::add_point error({})", Error::AddressNotFound.to_string());
                Err(Error::AddressNotFound)
            }
        }
        Err(e) =>{
            error!("dao::user::add_point error({:?})", e);
            Err(Error::InternalServerError)
        }
    }
}

/// 查询地址余额
pub async fn find_balance(pool: &MySqlPool, owner: String) -> DaoResult<i64> {
    let row = query(_SELECT_ADDRESS_POINT)
        .bind(owner)
        .fetch_one(pool)
        .await;

    match row {
        Ok(rs) => {
            Ok(rs.get("point"))
        }
        Err(sqlx::Error::RowNotFound) => {
            error!("dao::user::find_balance error({})", Error::AddressNotFound.to_string());
            Err(Error::AddressNotFound)
        }
        Err(e) => {
            error!("dao::user::find_balance error({:?})", e);
            Err(Error::InternalServerError)
        }
    }
}