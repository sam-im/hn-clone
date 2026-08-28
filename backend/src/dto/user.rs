use crate::{
    config::{
        ABOUT_MAX_LEN, PASSWORD_MAX_LEN, PASSWORD_MIN_LEN, PUBKEY_MAX_LEN, USERNAME_MAX_LEN,
        USERNAME_MIN_LEN,
    },
    error::{AppError, AppResult},
};

use super::{OptionalField, Validate, is_valid_charset, is_valid_len};

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
    pub password: OptionalField<String>,
    #[serde(default)]
    pub about: OptionalField<String>,
    #[serde(default)]
    pub pubkey: OptionalField<String>,
}

impl Validate for UpdateUserRequest {
    fn validate(&self) -> AppResult {
        if let OptionalField::Set(password) = &self.password {
            validate_password(password)?;
        }

        if let OptionalField::Set(about) = &self.about
            && !is_valid_len(about, &(None, Some(ABOUT_MAX_LEN)))
        {
            return Err(AppError::InvalidInput(format!(
                "About sections can not be larger than {ABOUT_MAX_LEN}."
            )));
        }
        // TODO: further validate to prevent XSS attacks / consider escaping before using on the frontend

        if let OptionalField::Set(pubkey) = &self.pubkey {
            validate_pubkey(pubkey)?;
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
pub fn validate_username(username: &str) -> AppResult {
    // [a-z0-9"-"]{4,36}
    // size
    if !is_valid_len(username, &(Some(USERNAME_MIN_LEN), Some(USERNAME_MAX_LEN))) {
        return Err(AppError::InvalidInput(format!(
            "Username length must be between {USERNAME_MIN_LEN} and {USERNAME_MAX_LEN} characters."
        )));
    }
    // charset
    let predicates = vec![
        |c: &char| -> bool { c.is_ascii_alphabetic() && c.is_ascii_lowercase() },
        |c: &char| -> bool { c.is_numeric() },
        |c: &char| -> bool { c.eq(&'-') },
    ];
    if !is_valid_charset(username, &predicates) {
        return Err(AppError::InvalidInput(format!(
            "Username must match: [a-z0-9\"-\"]{{{USERNAME_MIN_LEN},{USERNAME_MAX_LEN}}}"
        )));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> AppResult {
    // size
    let range = (Some(PASSWORD_MIN_LEN), Some(PASSWORD_MAX_LEN));
    if !is_valid_len(password, &range) {
        return Err(AppError::InvalidInput(format!(
            "Password length must be between {PASSWORD_MIN_LEN} and {PASSWORD_MAX_LEN} characters."
        )));
    }
    // char set
    let predicates = vec![
        |c: &char| -> bool { c.is_ascii_alphanumeric() },
        |c: &char| -> bool { c.is_ascii_punctuation() },
    ];
    if !is_valid_charset(password, &predicates) {
        return Err(AppError::InvalidInput(
            "Passwords can only consist of alphanumeric and punctuation.".to_string(),
        ));
    }
    // difficulty
    if !(password.contains(|c: char| c.is_numeric())
        && password.contains(|c: char| c.is_ascii_alphabetic())
        && password.contains(|c: char| c.is_ascii_punctuation()))
    {
        return Err(AppError::InvalidInput(
            "Passwords should contain at least one alphabetic, one numeric, and one punctuation character.".to_string()
        ));
    }
    Ok(())
}

/// Returns true if `pubkey` length does not exceed PUBKEY_MAX_LEN and is correctly parsed by the `pgp` crate.
pub fn validate_pubkey(pubkey: &str) -> AppResult {
    use pgp::composed::{Deserializable, SignedPublicKey};
    // size
    let range = (None, Some(PUBKEY_MAX_LEN));
    if !is_valid_len(pubkey, &range) {
        return Err(AppError::InvalidInput(format!(
            "Public keys can not be larger than {PUBKEY_MAX_LEN}."
        )));
    }
    // syntax
    let (_public_key, _headers_public) = SignedPublicKey::from_reader_single(pubkey.as_bytes())?;
    // TODO: check if semantic verification is required
    Ok(())
}
