use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::error::AppResult;
use crate::{config, error::AppError};

use super::{Validate, is_valid_charset, is_valid_len};

// NOTE:
// An example of validation for arbitrary text input:
// - Character length between 0 and X
// - No unicode characters, characters in english only

#[derive(Deserialize)]
pub struct NewPostRequest {
    pub title: String,
    pub content: String,
}

impl Validate for NewPostRequest {
    fn validate(&self) -> AppResult {
        // size
        let range = (None, (Some(config::POST_TITLE_LEN)));
        if !is_valid_len(&self.title, &range) {
            return Err(AppError::InvalidInputError(format!(
                "Post title can not be larger than {}.",
                config::POST_TITLE_LEN
            )));
        }
        if !is_valid_len(&self.title, &range) {
            return Err(AppError::InvalidInputError(format!(
                "Post content can not be larger than {}.",
                config::POST_CONTENT_LEN
            )));
        }
        // char set
        let predicates = vec![
            char::is_ascii_alphanumeric,
            char::is_ascii_punctuation,
            char::is_ascii_whitespace,
        ];
        if !is_valid_charset(&self.title, &predicates) {
            return Err(AppError::InvalidInputError(format!("TODO")));
        }
        if !is_valid_charset(&self.content, &predicates) {
            return Err(AppError::InvalidInputError(format!("TODO")));
        }
        Ok(())
    }
}

// TODO: impl. algo. similar to HN frontpage, with optional query for dates
// date format: dd/mm/yyyy
pub struct PopularPostsRequest {
    // date: Option<String>
}
impl Validate for PopularPostsRequest {
    fn validate(&self) -> AppResult {
        todo!()
    }
}

// TODO: impl. querying posts with optional query parameters:
// - title search (title="searched_title"),
// - sort direction (sort="asc" or "desc"*)
// - sort by (sort-by="score" or "date")
// - pagination (page=1*, per_page: 20*)
pub struct QueryPostsRequest {
    pub title_search: Option<()>,
    pub sort_by: Option<()>,
    pub sort_order: Option<()>,
    pub page: usize,
    pub per_page: usize,
}

impl Validate for QueryPostsRequest {
    fn validate(&self) -> AppResult {
        todo!()
    }
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub created_at: i64,
    pub comments: Vec<i32>,
    pub owner: String,
    pub title: String,
    pub content: String,
    pub upvotes: i64,
}

impl From<&Row> for PostResponse {
    fn from(value: &Row) -> Self {
        let id = value.get("_id");
        let created_at: DateTime<Utc> = value.get("_created_at");
        let created_at = created_at.timestamp();
        let owner = value.get("_owner");
        let upvotes = value.get("_upvotes");
        let title = value.get("_title");
        let content = value.get("_content");

        Self {
            id,
            upvotes,
            created_at,
            comments: vec![],
            owner,
            title,
            content,
        }
    }
}
