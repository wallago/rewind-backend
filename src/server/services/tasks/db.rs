use std::str::FromStr;

use anyhow::Result;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::{
        priorities::Priorities,
        status::Status,
        tasks::{NewTask, Task, UpdateTask},
    },
};

pub async fn get_all_tasks(pool: &DbPool) -> Result<Vec<Task>> {
    let recs = sqlx::query_as!(
        Task,
        r#"
            SELECT
                uuid, list_uuid, 
                name, description, position, deleted,
                status as "status: Status",
                priority as "priority: Priorities",
                created_at, updated_at, 
                deadline, start_date, finish_date            
            FROM tasks
            ORDER BY position
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
                uuid, list_uuid, 
                name, description, position, deleted,
                status as "status: Status",
                priority as "priority: Priorities",
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

pub async fn get_tasks_by_list_uuid(pool: &DbPool, list_uuid: String) -> Result<Vec<Task>> {
    let uuid = Uuid::from_str(&list_uuid)?;
    let recs = sqlx::query_as!(
        Task,
        r#"
            SELECT
                uuid, list_uuid, 
                name, description, 
                position, deleted,
                status as "status: Status",
                priority as "priority: Priorities",
                created_at, updated_at, 
                deadline, start_date, finish_date
            FROM tasks
            WHERE list_uuid = $1
            ORDER BY position
        "#,
        uuid
    )
    .fetch_all(pool)
    .await?;

    Ok(recs)
}

pub async fn insert_task(pool: &DbPool, new_task: NewTask) -> Result<Option<Task>> {
    if new_task.name.is_empty() || new_task.position < 0 {
        return Ok(None);
    }

    let rec = sqlx::query_as!(
        Task,
        r#"
            INSERT INTO tasks (list_uuid, name, description, status, priority, position)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                uuid, list_uuid, 
                name, description, position, deleted,
                status as "status: Status",
                priority as "priority: Priorities",
                created_at, updated_at,
                deadline, start_date, finish_date
        "#,
        new_task.list_uuid,
        new_task.name,
        new_task.description,
        new_task.status as Status,
        new_task.priority as Priorities,
        new_task.position
    )
    .fetch_one(pool)
    .await?;
    Ok(Some(rec))
}

pub async fn update_task(
    pool: &DbPool,
    task_uuid: String,
    updated_task: UpdateTask,
) -> Result<bool> {
    let uuid = Uuid::from_str(&task_uuid)?;
    let mut any_field = false;

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE tasks SET ");
    let mut separated = builder.separated(",");

    if let Some(name) = &updated_task.name {
        if !name.is_empty() {
            separated.push("name = ").push_bind_unseparated(name);
            any_field = true;
        }
    }

    if let Some(description) = &updated_task.description {
        separated
            .push("description = ")
            .push_bind_unseparated(description);
        any_field = true;
    }

    if let Some(status) = &updated_task.status {
        separated.push("status = ").push_bind_unseparated(status);
        any_field = true;
    }

    if let Some(priority) = &updated_task.priority {
        separated
            .push("priority = ")
            .push_bind_unseparated(priority);
        any_field = true;
    }

    if let Some(position) = &updated_task.position {
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

pub async fn switch_tasks_position(
    pool: &DbPool,
    task_uuid_from: String,
    task_uuid_to: String,
) -> Result<bool> {
    if task_uuid_to == task_uuid_from {
        return Ok(true);
    }

    let task_from = get_task_by_uuid(pool, task_uuid_from).await?;
    let task_to = get_task_by_uuid(pool, task_uuid_to).await?;

    let rows_affected = sqlx::query!(
        r#"UPDATE tasks SET position = -1 WHERE uuid = $1"#,
        task_from.uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected <= 0 {
        return Ok(false);
    }

    let rows_affected = sqlx::query!(
        r#"UPDATE tasks SET position = $1 WHERE uuid = $2"#,
        task_from.position,
        task_to.uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected <= 0 {
        return Ok(false);
    }

    let rows_affected = sqlx::query!(
        r#"UPDATE tasks SET position = $1 WHERE uuid = $2"#,
        task_to.position,
        task_from.uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    tracing::info!("{rows_affected}");

    Ok(rows_affected > 0)
}
