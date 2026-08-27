pub mod comment;
pub mod post;
pub mod session;
pub mod user;


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

}

    }
    Ok(())
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
