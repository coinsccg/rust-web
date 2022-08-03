use std::ops::Add;
use actix_web::{web::{self, Json, Data, Path, Payload, Bytes}, Result, route, HttpRequest};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::{mysql::MySqlPool, query, query_as, Row};
use std::time::{Duration, SystemTime};
use actix_web::dev::Service;
use serde::Serialize;
use serde_json;
use crate::constant;
use crate::errors::Error;
use crate::models::{Info, Password, Response, Point, ActiveUser};
use crate::auth::bearer_validator;
use crate::handlers::{login_handler, add_point_handler,active_user_handler};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(web::scope("/app")
                .route("/wallet", web::get().to(get_wallet)))
            .service(web::scope("/backend")
                .service(web::scope("/login").route("", web::post().to(login)))
                .service(web::scope("")
                    .wrap(HttpAuthentication::bearer(bearer_validator))
                    .wrap_fn(|req, srv|{
                        let fut = srv.call(req);
                        async {
                            let mut res = fut.await?;
                            res.headers_mut().insert("aa".as_str(), "ff".as_str());
                            Ok(res)
                        }
                    })
                    .route("/active", web::post().to(activate_user))
                    .route("/add_point", web::post().to(add_point))
                )
            )
    );
}

pub async fn get_wallet() -> Json<Response<String>> {
    Json(Response::success(constant::WALLET.to_string()))
}

pub async fn login(pool: Data<MySqlPool>, body: Bytes) -> Json<Response<String>> {
    let payload = std::str::from_utf8(body.as_ref()).map_err(|_|Error::RequestBadError);
    match payload {
        Ok(p) => {
            match serde_json::from_str::<Password>(p) {
                Ok(user) => {
                    match login_handler(pool.get_ref(), user).await {
                        Ok(token) => {
                            return Json(Response::success(token));
                        }
                        Err(e) => {
                            return Json(Response::fail(e.to_string(), String::new()));
                        }
                    }
                }
                Err(e) => {
                    return Json(Response::fail(e.to_string(), String::new()));
                }
            }
        }
        Err(e) => Json(Response::fail(e.to_string(), String::new()))
    }
}

pub async fn activate_user(pool: Data<MySqlPool>, body: Bytes, head: HttpRequest) -> Json<Response<String>> {
    println!("{:?}", head);
    let payload = std::str::from_utf8(body.as_ref()).map_err(|_|Error::RequestBadError);
    match payload {
        Ok(p) => {
            match serde_json::from_str::<ActiveUser>(p) {
                Ok(active) => {
                    match active_user_handler(pool.get_ref(), active.parent, active.owner).await {
                        Ok(_) => {
                            return Json(Response::success(String::new()))
                        }
                        Err(e) => {
                            return Json(Response::fail(e.to_string(), String::new()))
                        }
                    }
                }
                Err(e) => {
                    return Json(Response::fail(e.to_string(), String::new()))
                }
            }
        }
        Err(e) => {
            return Json(Response::fail(e.to_string(), String::new()))
        }
    }
}

pub async fn add_point(pool: Data<MySqlPool>, body: Bytes) -> Json<Response<String>> {
    let payload = std::str::from_utf8(body.as_ref()).map_err(|_|Error::RequestBadError);
    match payload {
        Ok(p) => {
            match serde_json::from_str::<Point>(p) {
                Ok(point) => {
                    match add_point_handler(pool.get_ref(), point.owner, point.point).await {
                        Ok(_) => {
                            return Json(Response::success(String::new()))
                        }
                        Err(e) => {
                            return Json(Response::fail(e.to_string(), String::new()))
                        }
                    }
                }
                Err(e) => {
                    return Json(Response::fail(e.to_string(), String::new()))
                }
            }
        }
        Err(e) => {
            return Json(Response::fail(e.to_string(), String::new()))
        }
    }
}
