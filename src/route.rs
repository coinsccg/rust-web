use actix_web::{web};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::auth::bearer_validator;
use crate::http;


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(web::scope("/app")
                .route("/wallet", web::get().to(http::get_wallet))
                .route("/balance", web::get().to(http::find_balance)))
            .service(web::scope("/backend")
                .service(web::scope("/login").route("", web::post().to(http::admin_login)))
                .service(web::scope("")
                    .wrap(HttpAuthentication::bearer(bearer_validator))
                    .route("/active", web::post().to(http::activate_user))
                    .route("/add_point", web::post().to(http::add_point))
                )
            )
    );
}

// response中间件
// use actix_web::http::header::{COOKIE, HeaderValue};
// .wrap_fn(|req, srv|{
//     let fut = srv.call(req);
//     async {
//         let mut res = fut.await?;
//         res.headers_mut().insert(COOKIE, HeaderValue::from(19));
//         Ok(res)
//     }
// })
