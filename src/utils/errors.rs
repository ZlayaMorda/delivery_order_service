use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("SQLx")]
    SQLxError(#[from] sqlx::Error),
}
