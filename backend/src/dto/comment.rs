use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::{
    dto::is_valid_charset,
    error::{AppError, AppResult},
};

use super::Validate;

#[derive(Deserialize)]
pub struct NewCommentRequest {
    pub parent: i32,
    pub content: String,
}

impl Validate for NewCommentRequest {
    fn validate(&self) -> AppResult {
        let predicates = vec![
            char::is_ascii_alphanumeric,
            char::is_ascii_punctuation,
            char::is_ascii_whitespace,
        ];
        if !is_valid_charset(&self.content, &predicates) {
            return Err(AppError::InvalidInputError("".to_string()));
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub created_at: i64,
    pub owner: String,
    pub parent: i32,
    pub children: Vec<i32>,
    pub upvotes: i64,
    pub content: String,
}

impl From<&Row> for CommentResponse {
    fn from(value: &Row) -> Self {
        let id = value.get("_id");
        let created_at: DateTime<Utc> = value.get("_created_at");
        let created_at = created_at.timestamp();
        let owner = value.get("_owner");
        let parent = value.get("_parent");
        let upvotes = value.get("_upvotes");
        let content = value.get("_content");

        Self {
            created_at,
            id,
            owner,
            parent,
            children: vec![],
            upvotes,
            content,
        }
    }
}
