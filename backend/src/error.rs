use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    InvalidInputError(String),
    #[error(transparent)]
    DatabasePoolError(#[from] deadpool_postgres::PoolError),
    #[error(transparent)]
    DatabaseError(#[from] tokio_postgres::Error),
    #[error("hash error: {0}")]
    HashError(String),
    #[error("resource already exists")]
    ResourceExistsError, // TODO: capture info about the resource
    #[error("resource not found")]
    ResourceNotFound, // TODO: capture info about the resource
    #[error("sessions error: {0}")]
    SessionError(String),
    #[error("auth error: {0}")]
    AuthError(String),
    #[error(transparent)]
    PgpError(#[from] pgp::errors::Error),
    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::HashError(value.to_string())
    }
}

impl AppError {
    pub fn response(self) -> (StatusCode, AppResponseError) {
        let message = self.to_string();
        let (kind, code, details, status_code) = match self {
            AppError::InvalidInputError(_) => (
                "INVALID_INPUT_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST,
            ),
            AppError::DatabasePoolError(_) => (
                "DATABASE_POOL_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::DatabaseError(_) => (
                "DATABASE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::HashError(_) => (
                "HASH_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::ResourceExistsError => (
                "RESOURCE_EXISTS_ERROR".to_string(),
                None,
                vec![],
                StatusCode::CONFLICT,
            ),
            AppError::SessionError(_) => (
                "SESSION_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::AuthError(_) => (
                "AUTH_ERROR".to_string(),
                None,
                vec![],
                StatusCode::UNAUTHORIZED,
            ),
            AppError::PgpError(_) => (
                "PGP_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST,
            ),
            AppError::ResourceNotFound => (
                "RESOURCE_NOT_FOUND".to_string(),
                None,
                vec![],
                StatusCode::NOT_FOUND,
            ),
            AppError::SystemTimeError(_) => (
                "SYSTEM_TIME_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };
        (
            status_code,
            AppResponseError::new(kind, message, code, details),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body) = self.response();
        (status_code, Json(body)).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct AppResponseError {
    pub kind: String,
    pub error_message: String,
    pub code: Option<i32>,
    pub details: Vec<(String, String)>,
}

impl AppResponseError {
    fn new(
        kind: impl Into<String>,
        message: impl Into<String>,
        code: Option<i32>,
        details: Vec<(String, String)>,
    ) -> Self {
        Self {
            kind: kind.into(),
            error_message: message.into(),
            code,
            details,
        }
    }
}
