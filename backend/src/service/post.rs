use deadpool_postgres::Object;

use crate::{
    dto::{
        PaginationParams, PaginationResponse,
        comment::CommentResponse,
        post::{NewPostRequest, PostResponse},
    },
    error::{AppError, AppResult},
    server::{session::Session, state::AppState},
    service::comments::retrieve_comments_by_parent,
};

pub async fn retrieve_post(state: AppState, id: i32) -> AppResult<PostResponse> {
    let db = state.db.get().await?;
    let post = query_post(&db, id).await?;
    Ok(post)
}

pub async fn create_post(
    state: AppState,
    session: Session,
    req: NewPostRequest,
) -> AppResult<PostResponse> {
    let mut db = state.db.get().await?;
    let tr = db.transaction().await?;

    // TODO: check user status after adding related attributes
    let statement = tr
        .prepare_cached("SELECT _username FROM _user WHERE _id = $1;")
        .await?;
    let rows = tr.query(&statement, &[&session.user_id]).await?;
    let _username: String = match rows.first() {
        Some(u) => u.get("_username"),
        None => return Err(AppError::AuthError("TODO".to_string())),
    };

    let statement = tr
        .prepare_cached("INSERT INTO _item (_type, _owner) VALUES ($1, $2) RETURNING _id;")
        .await?;
    let rows = tr.query(&statement, &[&"post", &session.user_id]).await?;
    let post_id: i32 = match rows.first() {
        Some(r) => r.get("_id"),
        None => {
            // TODO: add a better error type for this case
            return Err(AppError::InvalidInputError(
                "TODO: failed to insert item".to_string(),
            ));
        }
    };
    let statement = tr
        .prepare_cached("INSERT INTO _post (_id, _title, _content) VALUES ($1, $2, $3);")
        .await?;
    tr.execute(&statement, &[&post_id, &req.title, &req.content])
        .await?;
    tr.commit().await?;

    let post = query_post(&db, post_id).await?;
    Ok(post)
}

pub async fn retrieve_comments(
    state: AppState,
    id: i32,
    pagination: PaginationParams,
) -> AppResult<PaginationResponse<CommentResponse>> {
    retrieve_comments_by_parent(state, id, pagination).await
}

async fn query_post(db: &Object, id: i32) -> AppResult<PostResponse> {
    let post_stmt = db
        .prepare_cached(
            "SELECT _item._id AS _id,
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
            WHERE _item._id = $1;",
        )
        .await?;

    let post = match db.query_opt(&post_stmt, &[&id]).await? {
        Some(row) => PostResponse::from(&row),
        None => return Err(AppError::ResourceNotFoundError),
    };
    Ok(post)
}
