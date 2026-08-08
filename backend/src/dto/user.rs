use crate::{
    config::ABOUT_MAX_LEN,
    error::{AppError, AppResult},
};

use super::{
    UpdateField, Validate, is_valid_len, validate_password, validate_pubkey, validate_username,
};

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
}
impl Validate for RegisterUserRequest {
    fn validate(&self) -> AppResult {
        validate_username(&self.username)?;
        validate_password(&self.password)?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    #[serde(default)]
    pub password: UpdateField<String>,
    #[serde(default)]
    pub about: UpdateField<String>,
    #[serde(default)]
    pub pubkey: UpdateField<String>,
}

impl Validate for UpdateUserRequest {
    fn validate(&self) -> AppResult {
        if let UpdateField::Set(password) = &self.password {
            validate_password(&password)?;
        }

        if let UpdateField::Set(about) = &self.about {
            if !is_valid_len(&about, &(None, Some(ABOUT_MAX_LEN))) {
                return Err(AppError::InvalidInputError(format!(
                    "About sections can not be larger than {ABOUT_MAX_LEN}."
                )));
            }
            // TODO: further validate to prevent XSS attacks / consider escaping before using on the frontend
        }

        if let UpdateField::Set(pubkey) = &self.pubkey {
            validate_pubkey(&pubkey)?;
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    pub about: Option<String>,
    pub pubkey: Option<String>,
}

impl From<&Row> for UserResponse {
    fn from(value: &Row) -> Self {
        let username = value.get("_username");
        let about = value.get("_about");
        let pubkey = value.get("_public_key");

        Self {
            username,
            about,
            pubkey,
        }
    }
}
