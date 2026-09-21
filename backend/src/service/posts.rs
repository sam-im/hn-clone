use crate::{
    client::db::Database,
    dto::{
        PaginationParams, PaginationResponse,
        post::{PostResponse, SortMethod, SortOrder, SortingParams},
    },
    error::AppResult,
    server::state::AppState,
};

pub async fn retrieve_posts(
    state: AppState,
    pagination: PaginationParams,
    sorting: SortingParams,
) -> AppResult<PaginationResponse<PostResponse>> {
    let db = state.db.get().await?;

    let sort_by = match sorting.sort_by {
        SortMethod::Date => "_item._created_at",
        SortMethod::Vote => "_upvotes",
        SortMethod::Popular => {
            let posts = state.popular_posts.get(&pagination).await?;
            return Ok(posts);
        }
    };

    let sort_order = match sorting.sort_order {
        SortOrder::Asc => "ASC",
        SortOrder::Desc => "DESC",
    };

    // SAFETY: both `sort_by` and `sort_order` are &'static str
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
        sort_by, sort_order
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

pub async fn popular_posts(
    db: Database,
    pagination: &PaginationParams,
) -> AppResult<PaginationResponse<PostResponse>> {
    let db = db.get().await?;

    let tr = db
        .prepare_cached(
            "SELECT
                _item._id AS _id,
                _item._created_at AS _created_at,
                _user._username AS _owner,
                _upvotes_stats._count AS _upvotes,
                _comments_stats._count AS _comments,
                _post._title AS _title,
                _post._content AS _content,
                (
                    _upvotes_stats._count::numeric / POWER(
                        GREATEST(
                            EXTRACT(EPOCH FROM (NOW() - _item._created_at)) / 3600.0,
                            1
                        ) + 2,
                        1.8
                    )
                ) AS _score
                FROM _item
                    INNER JOIN _post ON _item._id = _post._id
                    INNER JOIN _user ON _item._owner = _user._id
                    CROSS JOIN LATERAL (
                        SELECT COUNT(DISTINCT _user) as _count
                        FROM _upvote
                        WHERE _upvote._item = _item._id
                    ) AS _upvotes_stats
                    CROSS JOIN LATERAL (
                        SELECT COUNT(*) AS _count
                        FROM _comment
                        WHERE _comment._parent = _item._id
                    ) AS _comments_stats
                WHERE _item._created_at >= NOW() - INTERVAL '7 days'
                ORDER BY _score DESC
                LIMIT $1 OFFSET $2;",
        )
        .await?;

    let posts = db
        .query(
            &tr,
            &[&(pagination.limit as i64), &(pagination.offset as i64)],
        )
        .await?
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
