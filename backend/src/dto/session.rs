use std::time::UNIX_EPOCH;

use axum_extra::headers::{Authorization, authorization::Bearer};
use serde::{Deserialize, Serialize};

use super::{
    Validate,
    user::{validate_password, validate_username},
};
use crate::{
    config::{SESSION_DURATIONS, SESSION_TOKEN_LEN},
    error::{AppError, AppResult},
    server::session::Session,
};

#[derive(Deserialize)]
pub struct SessionRequest {
    pub username: String,
    pub password: String,
    /// Duration in minutes.
    pub duration: u32,
}

impl Validate for SessionRequest {
    fn validate(&self) -> AppResult {
        // duration options: 1-hour, 8-hours, 1-day, and 1-week

        validate_username(&self.username)?;
        validate_password(&self.password)?;

        if !(SESSION_DURATIONS.contains(&self.duration)) {
            return Err(AppError::InvalidInputError(
                "duration must be one of {DURATIONS:?} minutes".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct TokenFromRequest {
    pub token: String,
}

impl Validate for TokenFromRequest {
    fn validate(&self) -> AppResult {
        if self.token.len() != SESSION_TOKEN_LEN {
            return Err(AppError::InvalidInputError(
                "Invalid token length.".to_string(),
            ));
        }
        if self.token.contains(|c: char| !c.is_ascii_alphanumeric()) {
            return Err(AppError::InvalidInputError(
                "Invalid characters in token.".to_string(),
            ));
        }
        Ok(())
    }
}

impl Validate for Authorization<Bearer> {
    fn validate(&self) -> AppResult {
        if self.token().len() != SESSION_TOKEN_LEN {
            return Err(AppError::InvalidInputError(
                "Invalid token length.".to_string(),
            ));
        }
        if self.token().contains(|c: char| !c.is_ascii_alphanumeric()) {
            return Err(AppError::InvalidInputError(
                "Invalid characters in token.".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct SessionResponse {
    pub issued_at: u64,
    pub expires_at: u64,
    pub username: String,
    pub token: String,
}

impl TryFrom<(String, Session, String)> for SessionResponse {
    type Error = AppError;

    fn try_from(value: (String, Session, String)) -> Result<Self, Self::Error> {
        let username = value.0;
        let issued_at = value.1.issued_at.duration_since(UNIX_EPOCH)?.as_secs();
        let expires_at = value.1.expires_at.duration_since(UNIX_EPOCH)?.as_secs();
        let token = value.2;
        Ok(Self {
            username,
            issued_at,
            expires_at,
            token,
        })
    }
}
