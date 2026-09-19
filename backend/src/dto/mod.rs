pub mod comment;
pub mod post;
pub mod session;
pub mod user;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    config,
    error::{AppError, AppResult},
};

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

#[derive(Clone, Copy)]
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
            Some(l) => {
                let l = l
                    .parse::<u32>()
                    .map_err(|e| AppError::InvalidInput(e.to_string()))?;
                if !(config::PAGINATION_LIMITS.contains(&l)) {
                    return Err(AppError::InvalidInput(format!(
                        "limit must be one of {:?}",
                        config::PAGINATION_LIMITS
                    )));
                }
                l
            }
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

pub enum SortMethod {
    Date,
    Vote,
    Popular,
}

impl SortMethod {
    pub fn to_sql_str(&self) -> &'static str {
        match self {
            SortMethod::Date => "_item._created_at",
            SortMethod::Vote => "_upvotes",
            _ => panic!(),
        }
    }
}

pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn to_sql_str(&self) -> &'static str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
}

pub struct SortingParams {
    pub sort_by: SortMethod,
    pub sort_order: SortOrder,
}

impl TryFrom<&HashMap<String, String>> for SortingParams {
    type Error = AppError;

    fn try_from(value: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let sort_by = match value.get("sort_by") {
            Some(m) => match m.as_str() {
                "date" => SortMethod::Date,
                "vote" => SortMethod::Vote,
                "popular" => SortMethod::Popular,
                _ => return Err(AppError::InvalidInput("invalid sorting method".to_string())),
            },
            None => SortMethod::Popular,
        };
        let sort_order = match value.get("sort_order") {
            Some(o) => match o.as_str() {
                "asc" => SortOrder::Asc,
                "desc" => SortOrder::Desc,
                _ => return Err(AppError::InvalidInput("invalid sorting order".to_string())),
            },
            None => SortOrder::Desc,
        };
        Ok(Self {
            sort_by,
            sort_order,
        })
    }
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
