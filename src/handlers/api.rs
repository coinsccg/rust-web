use std::ops::Add;
use std::time::{SystemTime, Duration};
use log::info;
use sqlx::mysql::MySqlPool;
use sqlx::{query, Row};
use crate::errors::Error;
use crate::auth::{Claims, encode_jwt};
use crate::models::Password;



type ResponseResult<T, E=Error> = Result<T, E>;

pub async fn login_handler(pool: &MySqlPool, user: Password) -> ResponseResult<String> {
    let rows = query(
        "SELECT id FROM users WHERE username = ? AND password = ?",
    ).bind(user.username).bind(user.password).fetch_one(pool).await;

    match rows {
        Ok(rs) => {
            if let Ok(id) = rs.try_get::<i64, _>("id"){
                println!("{:?}", id);
            }
        }
        Err(_) => {
            return Err(Error::LoginError);
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


pub async fn active_user_handler(pool: &MySqlPool, parent: String, owner: String) -> ResponseResult<()> {
    let rows = query(
        "SELECT id FROM points WHERE parent = ? AND owner = ?",
    )
        .bind(parent.clone())
        .bind(owner.clone())
        .fetch_one(pool)
        .await;


    match rows {
        Ok(rw) => {
            info!("111111111111111111111");
            Err(Error::UserAlreadyExists)
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
                        Err(Error::UserNotFound)
                    }
                }
                Err(e) =>{

                    Err(Error::DatabaseError)
                }
            }
        }
        Err(e) => {
            Err(Error::DatabaseError)
        }
    }
}

pub async fn add_point_handler(pool: &MySqlPool, owner: String, point: i64) -> ResponseResult<()> {
    let rows = query(
        "UPDATE points SET point = point + ? WHERE owner = ?"
    ).bind(point).bind(owner).execute(pool).await;
    match rows {
        Ok(rs) => {
            if rs.rows_affected() > 0 {
                Ok(())
            } else {
                Err(Error::UserNotFound)
            }
        }
        Err(e) =>{
            Err(Error::DatabaseError)
        }
    }
}

