pub mod database;
pub mod model;
pub mod http;
pub mod auth;
pub mod errors;
pub mod constant;
pub mod server;
pub mod service;
pub mod dao;
pub mod response;
pub mod route;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    server::run_server().await?;
    Ok(())
}
