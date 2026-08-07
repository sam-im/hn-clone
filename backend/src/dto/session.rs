use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};

use super::{Validate, validate_password, validate_username};
use crate::{
    config::{SESSION_DURATIONS, SESSION_TOKEN_LEN},
    error::AppError,
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
    fn validate(&self) -> Result<(), crate::error::AppError> {
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
    fn validate(&self) -> Result<(), AppError> {
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

#[derive(Serialize)]
pub struct SessionResponse {
    pub token: String,
    pub user_id: i32,
    pub issued_at: u64,
    pub expires_at: u64,
}

impl TryFrom<(String, Session)> for SessionResponse {
    type Error = AppError;

    fn try_from(value: (String, Session)) -> Result<Self, Self::Error> {
        let token = value.0;
        let user_id = value.1.user_id;
        let issued_at = value.1.issued_at.duration_since(UNIX_EPOCH)?.as_secs();
        let expires_at = value.1.expires_at.duration_since(UNIX_EPOCH)?.as_secs();
        Ok(Self {
            token,
            user_id,
            issued_at,
            expires_at,
        })
    }
}
