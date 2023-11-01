use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type CustomResult<T> = Result<T, CustomError>;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("auth error")]
    AuthError,
    #[error("jwt_error: {0}")]
    JWT(#[from] jsonwebtoken::errors::Error),
    #[error("db_error: {0}")]
    Db(#[from] sea_orm::DbErr),
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let code: StatusCode = match self {
            CustomError::AuthError => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let reason = self.to_string();
        tracing::error!("{} {}", code, reason);
        (code, reason).into_response()
    }
}
