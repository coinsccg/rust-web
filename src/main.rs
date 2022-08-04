pub mod database;
pub mod route;
pub mod models;
pub mod handlers;
pub mod auth;
pub mod errors;
pub mod constant;
pub mod server;
pub mod services;
pub mod parse;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    server::run_server().await?;
    Ok(())
}