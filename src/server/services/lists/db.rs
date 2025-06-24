use std::str::FromStr;

use anyhow::Result;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::{
        lists::{List, NewList, UpdateList},
        tasks::{Status, Task},
    },
};

pub async fn list_all_lists(pool: &DbPool) -> Result<Vec<List>> {
    let recs = sqlx::query_as!(
        List,
        r#"
            SELECT
                uuid, board_uuid, name, description, deleted,
                created_at, updated_at 
            FROM lists
            ORDER BY created_at
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(recs)
}

pub async fn get_list_by_uuid(pool: &DbPool, list_uuid: String) -> Result<List> {
    let uuid = Uuid::from_str(&list_uuid)?;
    let rec = sqlx::query_as!(
        List,
        r#"
            SELECT
                uuid, board_uuid, name, description, deleted,
                created_at, updated_at 
            FROM lists
            WHERE uuid = $1
        "#,
        uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn insert_list(pool: &DbPool, new_list: NewList) -> Result<List> {
    let rec = sqlx::query_as!(
        List,
        r#"
            INSERT INTO lists (board_uuid, name, description)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        new_list.board_uuid,
        new_list.name,
        new_list.description
    )
    .fetch_one(pool)
    .await?;
    Ok(rec)
}

pub async fn list_all_tasks_for_list(pool: &DbPool, list_uuid: String) -> Result<Vec<Task>> {
    let uuid = Uuid::from_str(&list_uuid)?;
    let recs = sqlx::query_as!(
        Task,
        r#"
            SELECT
                uuid, list_uuid, name, description, status as "status: Status", position, deleted,
                created_at, updated_at, deadline, start_date, finish_date
            FROM tasks
            WHERE list_uuid = $1
            ORDER BY created_at
        "#,
        uuid
    )
    .fetch_all(pool)
    .await?;
    Ok(recs)
}

pub async fn update_list(
    pool: &DbPool,
    list_uuid: String,
    updated_list: UpdateList,
) -> Result<bool> {
    let uuid = Uuid::from_str(&list_uuid)?;
    let mut any_field = false;

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE lists SET ");
    let mut separated = builder.separated(", ");

    if let Some(name) = &updated_list.name {
        separated.push("name = ").push_bind_unseparated(name);
        any_field = true
    }

    if let Some(description) = &updated_list.description {
        separated
            .push("description = ")
            .push_bind_unseparated(description);
        any_field = true
    }

    if !any_field {
        return Ok(false);
    }

    builder.push(" WHERE uuid = ").push_bind(uuid);

    let query = builder.build();
    let rows_affected = query.execute(pool).await?.rows_affected();

    Ok(rows_affected > 0)
}

pub async fn delete_list(pool: &DbPool, list_uuid: String) -> Result<bool> {
    let uuid = Uuid::from_str(&list_uuid)?;
    let rows_affected = sqlx::query!(
        r#"
            DELETE FROM lists
            WHERE uuid = $1
        "#,
        uuid
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}
