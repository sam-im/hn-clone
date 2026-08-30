pub mod comment;
pub mod post;
pub mod session;
pub mod user;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

pub trait Validate {
    fn validate(&self) -> AppResult;
}

/// Used with optional fields, where it may be specified,
/// specified but set to `null`, or unspecified.
///
/// Primary advantage over `Option` is that this allows us to
/// differentiate between a `null` and a missing field.
#[derive(Default, Deserialize)]
#[serde(untagged)]
pub enum OptionalField<T> {
    /// Field is present and set to something.
    Set(T),
    /// Field is present and set to null.
    Clear,
    /// Field is missing.
    #[default]
    Unspecified,
}

pub struct PaginationParams {
    pub offset: u32,
    pub limit: u32,
}

impl TryFrom<&HashMap<String, String>> for PaginationParams {
    type Error = AppError;

    fn try_from(value: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let offset = match value.get("offset") {
            Some(o) => o
                .parse::<u32>()
                .map_err(|e| AppError::InvalidInput(e.to_string()))?,
            None => 0,
        };
        let limit = match value.get("limit") {
            Some(l) => l
                .parse::<u32>()
                .map_err(|e| AppError::InvalidInput(e.to_string()))?,
            None => 20,
        };
        Ok(Self { offset, limit })
    }
}

#[derive(Serialize)]
pub struct PaginationResponse<T> {
    pub offset: Option<usize>,
    pub data: Vec<T>,
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
pub fn is_valid_charset<P: Fn(&char) -> bool>(input: &str, predicates: &[P]) -> bool {
    input
        .chars()
        .all(|c: char| predicates.iter().any(|predicate: &P| predicate(&c)))
}
