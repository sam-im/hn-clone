pub mod session;
pub mod user;

use serde::Deserialize;

use crate::{
    config::{
        PASSWORD_MAX_LEN, PASSWORD_MIN_LEN, PUBKEY_MAX_LEN, USERNAME_MAX_LEN, USERNAME_MIN_LEN,
    },
    error::{AppError, AppResult},
};

pub trait Validate {
    fn validate(&self) -> AppResult;
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
    // [a-z0-9"-"]{4,36}
    // size
    if !is_valid_len(username, &(Some(USERNAME_MIN_LEN), Some(USERNAME_MAX_LEN))) {
        return Err(AppError::InvalidInputError(format!(
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
        return Err(AppError::InvalidInputError(format!(
            "Username must match: [a-z0-9\"-\"]{{{USERNAME_MIN_LEN},{USERNAME_MAX_LEN}}}"
        )));
    }
    Ok(())
}
pub fn validate_password(password: &str) -> AppResult {
    // size
    let range = (Some(PASSWORD_MIN_LEN), Some(PASSWORD_MAX_LEN));
    if !is_valid_len(password, &range) {
        return Err(AppError::InvalidInputError(format!(
            "Password length must be between {PASSWORD_MIN_LEN} and {PASSWORD_MAX_LEN} characters."
        )));
    }
    // char set
    let predicates = vec![
        |c: &char| -> bool { c.is_ascii_alphanumeric() },
        |c: &char| -> bool { c.is_ascii_punctuation() },
    ];
    if !is_valid_charset(password, &predicates) {
        return Err(AppError::InvalidInputError(format!(
            "Passwords can only consist of alphanumeric and punctuation."
        )));
    }
    // difficulty
    if !(password.contains(|c: char| c.is_numeric())
        && password.contains(|c: char| c.is_ascii_alphabetic())
        && password.contains(|c: char| c.is_ascii_punctuation()))
    {
        return Err(AppError::InvalidInputError(format!(
            "Passwords should contain at least one alphabetic, one numeric, and one punctuation character."
        )));
    }
    Ok(())
}

/// Returns true if `pubkey` length does not exceed PUBKEY_MAX_LEN and is correctly parsed by the `pgp` crate.
pub fn validate_pubkey(pubkey: &str) -> AppResult {
    use pgp::composed::{Deserializable, SignedPublicKey};
    // size
    let range = (None, Some(PUBKEY_MAX_LEN));
    if !is_valid_len(pubkey, &range) {
        return Err(AppError::InvalidInputError(format!(
            "Public keys can not be larger than {PUBKEY_MAX_LEN}."
        )));
    }
    // syntax
    let (_public_key, _headers_public) = SignedPublicKey::from_reader_single(pubkey.as_bytes())?;
    // TODO: check if semantic verification is required
    Ok(())
}

/// Returns true if `input` length is between `range.0` (inclusive) and `range.1` (inclusive).
pub fn is_valid_len(input: &str, range: &(Option<usize>, Option<usize>)) -> bool {
    let len = input.len();
    let p1 = match range.0 {
        Some(start) => start <= len,
        None => true,
    };
    let p2 = match range.1 {
        Some(end) => len <= end,
        None => true,
    };
    p1 && p2
}

/// Returns true if at least one predicate in `predicates` returns true for all characters in `input`.
pub fn is_valid_charset<P: Fn(&char) -> bool>(input: &str, predicates: &Vec<P>) -> bool {
    input
        .chars()
        .map(|c: char| {
            predicates
                .iter()
                .map(|predicate: &P| predicate(&c))
                .any(|r: bool| r)
        })
        .all(|r: bool| r)
}
