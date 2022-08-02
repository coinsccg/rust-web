use actix_web::{web::{self, Data}, App, HttpServer, middleware::Logger};
use actix_cors::Cors;


mod db;
mod route;
mod types;
mod api;
mod middleware;
mod errors;
mod constant;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = db::init_db().await?;
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .configure(route::routes)
            .app_data(Data::new(pool.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
        .map_err(anyhow::Error::from) // 将std::io::Error转成anyhow::Error
}