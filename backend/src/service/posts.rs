use crate::{
    dto::{PaginationParams, PaginationResponse, SortingParams, post::PostResponse},
    error::AppResult,
    server::state::AppState,
};

pub async fn retrieve_posts(
    state: AppState,
    pagination: PaginationParams,
    sorting: SortingParams,
) -> AppResult<PaginationResponse<PostResponse>> {
    let db = state.db.get().await?;
    // SAFETY of the raw statement:
    // - both `sorting.sort_by` and `sorting.sort_order` are enums and,
    // - both calls to `to_sql_str(&self)` return `&'static str`
    let raw_stmt = format!(
        "SELECT
            _item._id AS _id,
            _item._created_at AS _created_at,
            _user._username AS _owner,
            (
                SELECT COUNT(DISTINCT _upvote._user)
                FROM _upvote
                WHERE _upvote._item = _item._id
            ) AS _upvotes,
            (
                SELECT COUNT(*)
                FROM _comment
                WHERE _comment._parent = _item._id
            ) AS _comments,
            _post._title AS _title,
            _post._content AS _content
        FROM _item
            INNER JOIN _post ON _item._id = _post._id
            INNER JOIN _user ON _user._id = _item._owner
        ORDER BY {} {}, _item._created_at DESC
        LIMIT $1 OFFSET $2;",
        sorting.sort_by.to_sql_str(),
        sorting.sort_order.to_sql_str()
    );
    let stmt = db.prepare_cached(&raw_stmt).await?;
    let rows = db
        .query(
            &stmt,
            &[&(pagination.limit as i64), &(pagination.offset as i64)],
        )
        .await?;
    let posts = rows
        .iter()
        .map(PostResponse::from)
        .collect::<Vec<PostResponse>>();

    let offset = if posts.len() == pagination.limit as usize {
        Some(pagination.offset as usize + posts.len())
    } else {
        None
    };
    let res = PaginationResponse {
        offset,
        data: posts,
    };
    Ok(res)
}
