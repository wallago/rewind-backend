use std::str::FromStr;

use anyhow::Result;
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::boards::{Board, NewBoard},
    models::lists::List,
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

// pub fn get_task_by_uuid(pool: &DbPool, task_uuid: String) -> Result<Task> {
//     let mut conn = pool.get()?;
//     // Ok(tasks::table
//     //     .filter(tasks::dsl::uuid.eq(task_uuid))
//     //     .first::<Task>(&mut *conn)?)
// }

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

// pub fn update_task(pool: &DbPool, task_uuid: String, mut updated_task: UpdateTask) -> Result<Task> {
//     let mut conn = pool.get()?;
//     updated_task.updated_at = Some(Utc::now().to_string());
//     diesel::update(tasks::table)
//         .filter(tasks::dsl::uuid.eq(&task_uuid))
//         .set(&updated_task)
//         .execute(&mut *conn)?;
//     get_task_by_uuid(pool, task_uuid)
// }

// pub fn delete_task(pool: &DbPool, task_uuid: String) -> Result<usize> {
//     let mut conn = pool.get()?;
//     Ok(diesel::delete(tasks::table)
//         .filter(tasks::dsl::uuid.eq(task_uuid))
//         .execute(&mut *conn)?)
// }
