use std::str::FromStr;

use anyhow::Result;
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::{
        boards::{Board, NewBoard, UpdateBoard},
        lists::List,
    },
};

pub async fn list_all_boards(pool: &DbPool) -> Result<Vec<Board>> {
    let recs = sqlx::query_as!(
        Board,
        r#"
            SELECT
                uuid, name, description, deleted,
                created_at, updated_at 
            FROM boards
            ORDER BY created_at
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
                uuid, name, description, deleted,
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
            INSERT INTO boards (name, description)
            VALUES ($1, $2)
            RETURNING *
        "#,
        new_board.name,
        new_board.description
    )
    .fetch_one(pool)
    .await?;
    Ok(rec)
}

pub async fn list_all_lists_for_board(pool: &DbPool, board_uuid: String) -> Result<Vec<List>> {
    let uuid = Uuid::from_str(&board_uuid)?;
    let recs = sqlx::query_as!(
        List,
        r#"
            SELECT
                uuid, board_uuid, name, description, deleted,
                created_at, updated_at 
            FROM lists
            WHERE board_uuid = $1
            ORDER BY created_at
        "#,
        uuid
    )
    .fetch_all(pool)
    .await?;
    Ok(recs)
}

pub async fn update_board(
    pool: &DbPool,
    board_uuid: String,
    updated_board: UpdateBoard,
) -> Result<bool> {
    let uuid = Uuid::from_str(&board_uuid)?;
    let rows_affected = sqlx::query!(
        r#"
            UPDATE boards
            SET name = $1, description = $2
            WHERE uuid = $3
        "#,
        updated_board.name,
        updated_board.description,
        uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

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
