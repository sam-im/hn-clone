use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    dto::{PaginationParams, PaginationResponse, SortingParams, post::PostResponse},
    error::AppResult,
    server::state::AppState,
    service::posts::retrieve_posts,
};

pub async fn get_posts(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> AppResult<(StatusCode, Json<PaginationResponse<PostResponse>>)> {
    let pagination = PaginationParams::try_from(&params)?;
    let sorting = SortingParams::try_from(&params)?;

    match retrieve_posts(state, pagination, sorting).await {
        Ok(resp) => Ok((StatusCode::OK, Json(resp))),
        Err(e) => Err(e),
    }
}
