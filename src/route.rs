use std::ops::Add;
use actix_web::{web::{self, Json, Data, Path, Payload, Bytes}, Result, route, HttpRequest};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::{mysql::MySqlPool, query, query_as, Row};
use std::time::{Duration, SystemTime};
use serde::Serialize;
use serde_json;
use crate::constant;
use crate::types::{Info, Password, Response};
use crate::middleware::{Claims, encode_jwt, bearer_validator};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(web::scope("/app")
                .route("/wallet", web::get().to(get_wallet)))
            .service(web::scope("/backend")
                .service(web::scope("/login").route("", web::post().to(login)))
                .service(web::scope("")
                    .wrap(HttpAuthentication::bearer(bearer_validator))
                    .route("/addPoint", web::get().to(add_point))
                )
            )
    );
}

pub async fn get_wallet() -> Json<Response<String>> {
    Json(Response::success(constant::WALLET.to_string()))
}

pub async fn login(pool: Data<MySqlPool>, body: Bytes) -> Json<Response<String>> {
    let pool = pool.get_ref();
    let payload = std::str::from_utf8(body.as_ref()).expect("payload error");
    if let Ok(user) = serde_json::from_str::<Password>(payload) {
        let rows = query(
            "SELECT id FROM users WHERE username = ? AND password = ?",
        ).bind(user.username).bind(user.password).fetch_one(pool).await;

        match rows {
            Ok(rs) => {
                if let Ok(id) = rs.try_get::<i64, _>("id") {
                    println!("{:?}", id);
                };
            }
            Err(e) => {
                return Json(Response::<String>::fail("请输入正确的用户名或密码".to_string()));
            }
        }

        let timestamp = SystemTime::now()
            .add(Duration::from_secs(1200))
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();
        let claims = Claims::new(1, timestamp);
        let token = encode_jwt(claims).unwrap();
        Json(Response::<String>::success(token))
    } else {
        Json(Response::<String>::fail("请输入正确的用户名或密码".to_string()))
    }

}

pub async fn add_point(info: Json<Info>) -> Result<String> {

    Ok(format!(
        "Welcome home {}, user_id {}!!",
        info.friend, info.user_id
    ))
}
