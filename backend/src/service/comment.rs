use deadpool_postgres::Object;

use crate::{
    dto::comment::{CommentResponse, NewCommentRequest},
    error::{AppError, AppResult},
    server::{session::Session, state::AppState},
};

pub async fn create_comment(
    state: AppState,
    session: Session,
    new_comment: NewCommentRequest,
) -> AppResult<CommentResponse> {
    let mut db = state.db.get().await?;
    let tr = db.transaction().await?;
    {
        // TODO: impl. parent item checks here
        let parent_stmt = tr
            .prepare_cached("SELECT _type FROM _item WHERE _id = $1;")
            .await?;
        let row = tr.query_opt(&parent_stmt, &[&new_comment.parent]).await?;
        let _kind: String = match row {
            Some(p) => p.get("_type"),
            None => return Err(AppError::InvalidInputError("invalid parent".to_string())),
        };
    }
    let id: i32;
    {
        let item_stmt = tr
            .prepare_cached("INSERT INTO _item (_type, _owner) VALUES ($1, $2) RETURNING _id;")
            .await?;
        let row = tr
            .query_one(&item_stmt, &[&"comment", &session.user_id])
            .await?;
        id = row.get(0);
        let comment_stmt = tr
            .prepare_cached("INSERT INTO _comment (_id, _parent, _content) VALUES ($1, $2, $3);")
            .await?;
        tr.execute(
            &comment_stmt,
            &[&id, &new_comment.parent, &new_comment.content],
        )
        .await?;
    }
    tr.commit().await?;
    let comment = query_comment(&db, id).await?;
    Ok(comment)
}

pub async fn retrieve_comment(state: AppState, id: i32) -> AppResult<CommentResponse> {
    let db = state.db.get().await?;
    let comment = query_comment(&db, id).await?;
    Ok(comment)
}

async fn query_comment(db: &Object, id: i32) -> AppResult<CommentResponse> {
    let comment_stmt = db
        .prepare_cached(
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
            WHERE _item._id = $1;",
        )
        .await?;

    let comment = match db.query_opt(&comment_stmt, &[&id]).await? {
        Some(row) => CommentResponse::from(&row),
        None => return Err(AppError::ResourceNotFoundError),
    };
    Ok(comment)
}
