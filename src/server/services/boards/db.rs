use std::str::FromStr;

use anyhow::Result;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::boards::{Board, NewBoard, UpdateBoard},
    utils::get_next_available_position,
};

pub async fn get_all_boards(pool: &DbPool) -> Result<Vec<Board>> {
    let recs = sqlx::query_as!(
        Board,
        r#"
            SELECT
                uuid, 
                name, description, position, deleted,
                created_at, updated_at
            FROM boards
            ORDER BY position
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(recs)
}

pub async fn get_board_by_uuid(pool: &DbPool, board_uuid: String) -> Result<Board> {
    let uuid = Uuid::from_str(&board_uuid)?;
    let rec = sqlx::query_as!(
        Board,
        r#"
            SELECT
                uuid, 
                name, description, position, deleted,
                created_at, updated_at 
            FROM boards
            WHERE uuid = $1
        "#,
        uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn insert_board(pool: &DbPool, new_board: NewBoard) -> Result<Option<Board>> {
    if new_board.name.is_empty() {
        return Ok(None);
    }

    let position = match new_board.position {
        Some(position) => {
            if position < 0 {
                return Ok(None);
            } else {
                position
            }
        }
        None => {
            let boards = get_all_boards(pool).await?;
            let used_positions: Vec<i32> = boards.iter().map(|board| board.position).collect();
            get_next_available_position(used_positions)
        }
    };

    let rec = sqlx::query_as!(
        Board,
        r#"
            INSERT INTO boards (name, description, position)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        new_board.name,
        new_board.description,
        Some(position)
    )
    .fetch_one(pool)
    .await?;
    Ok(Some(rec))
}

pub async fn update_board(
    pool: &DbPool,
    board_uuid: String,
    updated_board: UpdateBoard,
) -> Result<bool> {
    let uuid = Uuid::from_str(&board_uuid)?;
    let mut any_field = false;

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE boards SET ");
    let mut separated = builder.separated(", ");

    if let Some(name) = &updated_board.name {
        if !name.is_empty() {
            separated.push("name = ").push_bind_unseparated(name);
            any_field = true;
        }
    }

    if let Some(description) = &updated_board.description {
        separated
            .push("description = ")
            .push_bind_unseparated(description);
        any_field = true;
    }

    if let Some(position) = &updated_board.position {
        if !position < 0 {
            separated
                .push("position = ")
                .push_bind_unseparated(position);
            any_field = true;
        }
    }

    if !any_field {
        return Ok(false);
    }

    builder.push(" WHERE uuid = ").push_bind(uuid);

    let query = builder.build();
    let rows_affected = query.execute(pool).await?.rows_affected();

    Ok(rows_affected > 0)
}

pub async fn delete_board(pool: &DbPool, board_uuid: String) -> Result<bool> {
    let uuid = Uuid::from_str(&board_uuid)?;
    let rows_affected = sqlx::query!(
        r#"
            DELETE FROM boards
            WHERE uuid = $1
        "#,
        uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn switch_boards_position(
    pool: &DbPool,
    board_uuid_from: String,
    board_uuid_to: String,
) -> Result<bool> {
    if board_uuid_to == board_uuid_from {
        return Ok(true);
    }

    let board_from = get_board_by_uuid(pool, board_uuid_from).await?;
    let board_to = get_board_by_uuid(pool, board_uuid_to).await?;

    let rows_affected = sqlx::query!(
        r#"UPDATE boards SET position = -1 WHERE uuid = $1"#,
        board_from.uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected <= 0 {
        return Ok(false);
    }

    let rows_affected = sqlx::query!(
        r#"UPDATE boards SET position = $1 WHERE uuid = $2"#,
        board_from.position,
        board_to.uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected <= 0 {
        return Ok(false);
    }

    let rows_affected = sqlx::query!(
        r#"UPDATE boards SET position = $1 WHERE uuid = $2"#,
        board_to.position,
        board_from.uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    tracing::info!("{rows_affected}");

    Ok(rows_affected > 0)
}
