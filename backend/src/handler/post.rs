use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tracing::warn;

use crate::{
    dto::{
        Validate,
        post::{NewPostRequest, PostResponse},
    },
    error::AppResult,
    server::{session::verify_session, state::AppState},
    service::post::{create_post, retrieve_post},
};

pub async fn post_post(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(body): Json<NewPostRequest>,
) -> AppResult<(StatusCode, Json<PostResponse>)> {
    token.validate()?;
    let session = verify_session(&state.sessions, token.token()).await?;
    body.validate()?;

    match create_post(state, session, body).await {
        Ok(resp) => Ok((StatusCode::CREATED, Json(resp))),
        Err(e) => {
            warn!("{}", e);
            Err(e)
        }
    }
}
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<(StatusCode, Json<PostResponse>)> {
    match retrieve_post(state, id).await {
        Ok(resp) => Ok((StatusCode::OK, Json(resp))),
        Err(e) => Err(e),
    }
}

// TODO: consider moving to posts.rs
pub async fn get_posts(State(_state): State<AppState>) {
    todo!()
}
