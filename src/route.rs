use actix_web::{web};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::auth::bearer_validator;
use crate::handlers::{
    login_handler,
    get_wallet_handler,
    activate_user_handler,
    add_point_handler,
    find_balance_handler,
};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(web::scope("/app")
                .route("/wallet", web::get().to(get_wallet_handler))
                .route("/balance", web::get().to(find_balance_handler)))
            .service(web::scope("/backend")
                .service(web::scope("/login").route("", web::post().to(login_handler)))
                .service(web::scope("")
                    .wrap(HttpAuthentication::bearer(bearer_validator))
                    .route("/active", web::post().to(activate_user_handler))
                    .route("/add_point", web::post().to(add_point_handler))
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
