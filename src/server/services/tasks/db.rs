use std::str::FromStr;

use anyhow::Result;
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::tasks::{NewTask, Status, Task, UpdateTask},
};

pub async fn list_all_tasks(pool: &DbPool) -> Result<Vec<Task>> {
    let recs = sqlx::query_as!(
        Task,
        r#"
            SELECT
                uuid, list_uuid, name, description, status as "status: Status", position, deleted,
                created_at, updated_at, deadline, start_date, finish_date
            FROM tasks
            ORDER BY created_at
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(recs)
}

pub async fn get_task_by_uuid(pool: &DbPool, task_uuid: String) -> Result<Task> {
    let uuid = Uuid::from_str(&task_uuid)?;
    let rec = sqlx::query_as!(
        Task,
        r#"
            SELECT
                uuid, list_uuid, name, description,
                status as "status: Status",
                position, deleted,
                created_at, updated_at,
                deadline, start_date, finish_date
            FROM tasks
            WHERE uuid = $1
        "#,
        uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn insert_task(pool: &DbPool, new_task: NewTask) -> Result<Task> {
    let rec = sqlx::query_as!(
        Task,
        r#"
            INSERT INTO tasks (list_uuid, name, description, status, position)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                uuid, list_uuid, name, description,
                status as "status: Status",
                position, deleted,
                created_at, updated_at,
                deadline, start_date, finish_date
        "#,
        new_task.list_uuid,
        new_task.name,
        new_task.description,
        new_task.status as Status,
        new_task.position
    )
    .fetch_one(pool)
    .await?;
    Ok(rec)
}

pub async fn update_task(
    pool: &DbPool,
    task_uuid: String,
    updated_task: UpdateTask,
) -> Result<bool> {
    let uuid = Uuid::from_str(&task_uuid)?;
    let rows_affected = sqlx::query!(
        r#"
            UPDATE tasks
            SET name = $1, description = $2, status = $3, position = $4
            WHERE uuid = $5
        "#,
        updated_task.name,
        updated_task.description,
        updated_task.status as Option<Status>,
        updated_task.position,
        uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn delete_task(pool: &DbPool, task_uuid: String) -> Result<bool> {
    let uuid = Uuid::from_str(&task_uuid)?;
    let rows_affected = sqlx::query!(
        r#"
            DELETE FROM tasks
            WHERE uuid = $1
        "#,
        uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}
