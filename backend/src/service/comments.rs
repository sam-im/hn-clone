use crate::{
    dto::{
        PaginationParams, PaginationResponse,
        comment::{CommentResponse, SortBy},
    },
    error::AppResult,
    server::state::AppState,
};

pub async fn retrieve_comments_by_parent(
    state: AppState,
    parent_id: i32,
    pagination: PaginationParams,
    sort_by: SortBy,
) -> AppResult<PaginationResponse<CommentResponse>> {
    let db = state.db.get().await?;

    let sort_by_sql = match sort_by {
        SortBy::Newest => "_item._created_at DESC",
        SortBy::Oldest => "_item._created_at ASC",
    };
    // SAFETY: `sort_by_sql` is a &'static str
    let raw_stmt = format!(
        "SELECT
            _item._id AS _id,
            _item._created_at AS _created_at,
            _user._username AS _owner,
            _comment._parent AS _parent,
            (
                SELECT COUNT(DISTINCT _upvote._user)
                FROM _upvote
                WHERE _upvote._item = _item._id
            ) AS _upvotes,
            (
                SELECT COUNT(*)
                FROM _comment
                WHERE _comment._parent = _item._id
            ) AS _replies,
            _comment._content AS _content
        FROM _item
        INNER JOIN _comment ON _item._id = _comment._id
        INNER JOIN _user ON _item._owner = _user._id
        WHERE _comment._parent = $1
        ORDER BY {}
        LIMIT $2 OFFSET $3;",
        sort_by_sql
    );
    let stmt = db.prepare_cached(&raw_stmt).await?;

    let comments = db
        .query(
            &stmt,
            &[
                &parent_id,
                &(pagination.limit as i64),
                &(pagination.offset as i64),
            ],
        )
        .await?
        .iter()
        .map(CommentResponse::from)
        .collect::<Vec<CommentResponse>>();

    let offset = if comments.len() == pagination.limit as usize {
        Some(pagination.offset as usize + comments.len())
    } else {
        None
    };
    let res = PaginationResponse {
        offset,
        data: comments,
    };
    Ok(res)
}
