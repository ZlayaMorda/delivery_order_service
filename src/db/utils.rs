use std::time::Duration;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgPoolOptions};
use crate::utils::errors::AppError;

pub async fn get_pool(url: &str) -> Result<Pool<Postgres>, AppError> {
    Ok(
        PgPoolOptions::new()
            .max_connections(2)
            .acquire_timeout(Duration::from_secs(3))
            .connect(url)
            .await?
    )
}