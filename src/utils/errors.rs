use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("SQLx")]
    SQLxError(#[from] sqlx::Error),
    // #[error("GraphQL")]
    // GraphQLError(#[from] async_graphql::Error),
    #[error("Permission denied: {0}")]
    PermissionError(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("App state error: {0}")]
    AppError(String),
    #[error("Unauthorized, invalid token")]
    Unauthorized(),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = match self {
            AppError::SQLxError(_) => {StatusCode::INTERNAL_SERVER_ERROR}
            AppError::PermissionError(_) => {StatusCode::FORBIDDEN}
            AppError::BadRequest(_) => {StatusCode::BAD_REQUEST}
            AppError::AppError(_) => {StatusCode::INTERNAL_SERVER_ERROR}
            AppError::Unauthorized() => {StatusCode::BAD_REQUEST}
        };
        (
            code,
            format!("Something went wrong: {}", self),
        ).into_response()
    }
}