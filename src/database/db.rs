use sqlx::{self, MySqlPool};
use crate::constant::DBURL;

pub async fn init_db() -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPool::connect(DBURL).await?;
    Ok(pool)
}