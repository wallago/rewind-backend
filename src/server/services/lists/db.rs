use std::str::FromStr;

use anyhow::Result;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::lists::{List, NewList, UpdateList},
};

pub async fn get_all_lists(pool: &DbPool) -> Result<Vec<List>> {
    let recs = sqlx::query_as!(
        List,
        r#"
            SELECT
                uuid, board_uuid, 
                name, description, position, deleted,
                created_at, updated_at 
            FROM lists
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
                uuid, board_uuid, 
                name, description, position, deleted,
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

pub async fn get_lists_by_board_uuid(pool: &DbPool, board_uuid: String) -> Result<List> {
    let uuid = Uuid::from_str(&board_uuid)?;
    let rec = sqlx::query_as!(
        List,
        r#"
            SELECT
                uuid, board_uuid, 
                name, description, 
                position, deleted,
                created_at, updated_at 
            FROM lists
            WHERE board_uuid = $1
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
            INSERT INTO lists (board_uuid, name, description, position)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
        new_list.board_uuid,
        new_list.name,
        new_list.description,
        new_list.position
    )
    .fetch_one(pool)
    .await?;
    Ok(rec)
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

    if let Some(position) = &updated_list.position {
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
