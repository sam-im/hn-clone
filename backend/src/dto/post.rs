use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::error::AppResult;
use crate::{config, error::AppError};

use super::{Validate, is_valid_charset, is_valid_len};

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
            return Err(AppError::InvalidInput(format!(
                "Post title can not be larger than {}.",
                config::POST_TITLE_LEN
            )));
        }
        if !is_valid_len(&self.title, &range) {
            return Err(AppError::InvalidInput(format!(
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
            return Err(AppError::InvalidInput(
                "Invalid character(s) in post title.".to_string(),
            ));
        }
        if !is_valid_charset(&self.content, &predicates) {
            return Err(AppError::InvalidInput(
                "Invalid character(s) in post body.".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub created_at: i64,
    pub comments: i64,
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
        let comments = value.get("_comments");

        Self {
            id,
            upvotes,
            created_at,
            comments,
            owner,
            title,
            content,
        }
    }
}
