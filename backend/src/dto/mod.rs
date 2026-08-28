pub mod comment;
pub mod post;
pub mod session;
pub mod user;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::AppResult;

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

#[derive(Deserialize)]
pub struct PaginationParams {
    pub offset: u32,
    pub limit: u32,
}

impl From<HashMap<String, u32>> for PaginationParams {
    fn from(value: HashMap<String, u32>) -> Self {
        let offset = value.get("offset").unwrap_or(&0).to_owned();
        let limit = value.get("limit").unwrap_or(&20).to_owned();
        Self { offset, limit }
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
