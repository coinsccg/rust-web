use actix_web::{
    web::Data,
    App,
    HttpServer,
    middleware::Logger
};
use actix_cors::Cors;
use crate::database;
use crate::route;

pub async fn run_server() -> anyhow::Result<()>{
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let pool = database::init_db().await?;

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
        .map_err(anyhow::Error::from)
}