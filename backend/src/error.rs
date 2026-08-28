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
    InvalidInput(String),
    #[error(transparent)]
    DatabasePool(#[from] deadpool_postgres::PoolError),
    #[error(transparent)]
    Database(#[from] tokio_postgres::Error),
    #[error("hash error: {0}")]
    Hash(String),
    #[error("resource already exists")]
    ResourceExists, // TODO: capture info about the resource
    #[error("resource not found")]
    ResourceNotFound, // TODO: capture info about the resource
    #[error("resource not modified")]
    ResourceNotModified, // TODO: capture info about the resource
    #[error("sessions error: {0}")]
    Session(String),
    #[error("auth error: {0}")]
    Auth(String),
    #[error(transparent)]
    Pgp(#[from] pgp::errors::Error),
    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::Hash(value.to_string())
    }
}

impl AppError {
    pub fn response(self) -> (StatusCode, AppResponseError) {
        let message = self.to_string();
        let (kind, code, details, status_code) = match self {
            AppError::InvalidInput(_) => (
                "INVALID_INPUT_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST,
            ),
            AppError::DatabasePool(_) => (
                "DATABASE_POOL_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::Database(_) => (
                "DATABASE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::Hash(_) => (
                "HASH_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::ResourceExists => (
                "RESOURCE_EXISTS_ERROR".to_string(),
                None,
                vec![],
                StatusCode::CONFLICT,
            ),
            AppError::Session(_) => (
                "SESSION_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::Auth(_) => (
                "AUTH_ERROR".to_string(),
                None,
                vec![],
                StatusCode::UNAUTHORIZED,
            ),
            AppError::Pgp(_) => (
                "PGP_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST,
            ),
            AppError::SystemTime(_) => (
                "SYSTEM_TIME_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::ResourceNotFound => (
                "RESOURCE_NOT_FOUND_ERROR".to_string(),
                None,
                vec![],
                StatusCode::NOT_FOUND,
            ),
            AppError::ResourceNotModified => (
                "RESOURCE_NOT_MODIFIED".to_string(),
                None,
                vec![],
                StatusCode::NOT_MODIFIED,
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
