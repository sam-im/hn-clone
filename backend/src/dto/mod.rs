pub mod session;
pub mod user;

use serde::Deserialize;

use crate::{
    config::{
        PASSWORD_MAX_LEN, PASSWORD_MIN_LEN, PASSWORD_SPECIAL_CHARS, PUBKEY_MAX_LEN,
        USERNAME_MAX_LEN, USERNAME_MIN_LEN,
    },
    error::{AppError, AppResult},
};

pub trait Validate {
    fn validate(&self) -> Result<(), AppError>;
}

/// Used with PATCH requests, where a field may be specified,
/// specified but set to null, or unspecified.
#[derive(Default, Deserialize)]
#[serde(untagged)]
pub enum UpdateField<T> {
    /// Field is present and set to something.
    Set(T),
    /// Field is present and set to null.
    Clear,
    /// Field is missing.
    #[default]
    Unspecified,
}

pub fn validate_username(username: &str) -> AppResult {
    if !(username.len() >= USERNAME_MIN_LEN && username.len() <= USERNAME_MAX_LEN) {
        return Err(AppError::InvalidInputError(format!(
            "Username length must be between {USERNAME_MIN_LEN} and {USERNAME_MAX_LEN} characters."
        )));
    }
    if username.contains(|c: char| !c.is_ascii_alphanumeric()) {
        return Err(AppError::InvalidInputError(format!(
            "Usernames can only consist of alphanumeric ascii characters."
        )));
    }
    Ok(())
}
pub fn validate_password(password: &str) -> AppResult {
    let special_chars: String = PASSWORD_SPECIAL_CHARS.iter().copied().collect();

    if !(password.len() >= PASSWORD_MIN_LEN && password.len() <= PASSWORD_MAX_LEN) {
        return Err(AppError::InvalidInputError(format!(
            "Password length must be between {PASSWORD_MIN_LEN} and {PASSWORD_MAX_LEN} characters."
        )));
    }
    if password.contains(|c: char| !c.is_alphanumeric() && !PASSWORD_SPECIAL_CHARS.contains(&c)) {
        let special_chars: String = PASSWORD_SPECIAL_CHARS.iter().copied().collect();
        return Err(AppError::InvalidInputError(format!(
            "Passwords can only consist of alphanumeric and special characters ({special_chars})."
        )));
    }
    if !(password.contains(char::is_alphabetic)
        && password.contains(char::is_numeric)
        && password.contains(|c: char| PASSWORD_SPECIAL_CHARS.contains(&c)))
    {
        return Err(AppError::InvalidInputError(format!(
            "Passwords should contain at least one alphabetic, one numeric, and one special character ({special_chars})."
        )));
    }
    Ok(())
}

pub fn validate_pubkey(pubkey: &str) -> AppResult {
    use pgp::composed::{Deserializable, SignedPublicKey};
    if pubkey.len() > PUBKEY_MAX_LEN {
        return Err(AppError::InvalidInputError(format!(
            "Public keys can not be larger than {PUBKEY_MAX_LEN}."
        )));
    }
    let (_public_key, _headers_public) = SignedPublicKey::from_reader_single(pubkey.as_bytes())?;
    // TODO: check if semantic verification is required
    Ok(())
}
