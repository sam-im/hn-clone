use crate::error::{AppError, AppResult};

use super::Validate;

use serde::{Deserialize, Serialize};

// TODO: move constants to mod.rs
const USERNAME_MIN_LEN: usize = 4;
const USERNAME_MAX_LEN: usize = 36;
const PASSWORD_MIN_LEN: usize = 8;
const PASSWORD_MAX_LEN: usize = 64;
const ABOUT_MAX_LEN: usize = 256;
const PUBKEY_MAX_LEN: usize = 64 * 1024;
const SPECIAL_CHARS: &[char] = &[
    '`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{',
    '}', '\\', '|', ';', ':', '\'', '"', ',', '.', '/', '<', '>', '?',
];

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

/// Used with PATCH requests, where a field may be specified,
/// specified but set to null, or unspecified.
#[derive(Default, Deserialize)]
pub enum UpdateField<T> {
    /// Field is missing.
    #[default]
    Unspecified,
    /// Field is present and set to something.
    Set(T),
    /// Field is present and set to null.
    Clear,
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
            if about.len() > ABOUT_MAX_LEN {
                return Err(AppError::InvalidInputError(format!(
                    "About sections can not be larger than {ABOUT_MAX_LEN}."
                )));
            }
            // TODO: further validate to prevent XSS attacks
        }

        if let UpdateField::Set(pubkey) = &self.pubkey {
            validate_pubkey(&pubkey)?;
        }
        Ok(())
    }
}

fn validate_username(username: &str) -> AppResult {
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
fn validate_password(password: &str) -> AppResult {
    let special_chars: String = SPECIAL_CHARS.iter().copied().collect();

    if !(password.len() >= PASSWORD_MIN_LEN && password.len() <= PASSWORD_MAX_LEN) {
        return Err(AppError::InvalidInputError(format!(
            "Password length must be between {PASSWORD_MIN_LEN} and {PASSWORD_MAX_LEN} characters."
        )));
    }
    if password.contains(|c: char| !c.is_alphanumeric() && !SPECIAL_CHARS.contains(&c)) {
        let special_chars: String = SPECIAL_CHARS.iter().copied().collect();
        return Err(AppError::InvalidInputError(format!(
            "Passwords can only consist of alphanumeric and special characters ({special_chars})."
        )));
    }
    if !(password.contains(char::is_alphabetic)
        && password.contains(char::is_numeric)
        && password.contains(|c: char| SPECIAL_CHARS.contains(&c)))
    {
        return Err(AppError::InvalidInputError(format!(
            "Passwords should contain at least one alphabetic, one numeric, and one special character ({special_chars})."
        )));
    }
    Ok(())
}

fn validate_pubkey(pubkey: &str) -> AppResult {
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

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub id: i32,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    pub about: Option<String>,
    // pub karma: usize,
    pub pubkey: Option<String>,
}
