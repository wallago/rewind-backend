use std::str::FromStr;

use anyhow::Result;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::boards::{Board, NewBoard, UpdateBoard},
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

pub async fn insert_board(pool: &DbPool, new_board: NewBoard) -> Result<Board> {
    let rec = sqlx::query_as!(
        Board,
        r#"
            INSERT INTO boards (name, description, position)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        new_board.name,
        new_board.description,
        new_board.position
    )
    .fetch_one(pool)
    .await?;
    Ok(rec)
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
        separated.push("name = ").push_bind_unseparated(name);
        any_field = true;
    }

    if let Some(description) = &updated_board.description {
        separated
            .push("description = ")
            .push_bind_unseparated(description);
        any_field = true;
    }

    if let Some(position) = &updated_board.position {
        separated
            .push("position = ")
            .push_bind_unseparated(position);
        any_field = true;
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
