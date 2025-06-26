use std::str::FromStr;

use anyhow::Result;
use regex::Regex;
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::tags::{NewTag, Tag},
};

pub async fn get_all_tags(pool: &DbPool) -> Result<Vec<Tag>> {
    let recs = sqlx::query_as!(
        Tag,
        r#"
            SELECT
                uuid, 
                name, color
            FROM tags
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(recs)
}

pub async fn get_tag_by_uuid(pool: &DbPool, tag_uuid: String) -> Result<Tag> {
    let uuid = Uuid::from_str(&tag_uuid)?;
    let rec = sqlx::query_as!(
        Tag,
        r#"
            SELECT
                uuid, 
                name, color
            FROM tags
            WHERE uuid = $1
        "#,
        uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

// pub async fn get_tasks_by_list_uuid(pool: &DbPool, list_uuid: String) -> Result<Vec<Task>> {
//     let uuid = Uuid::from_str(&list_uuid)?;
//     let recs = sqlx::query_as!(
//         Task,
//         r#"
//             SELECT
//                 uuid, list_uuid,
//                 name, description,
//                 position, deleted,
//                 status as "status: Status",
//                 priority as "priority: Priorities",
//                 created_at, updated_at,
//                 deadline, start_date, finish_date
//             FROM tasks
//             WHERE list_uuid = $1
//             ORDER BY position
//         "#,
//         uuid
//     )
//     .fetch_all(pool)
//     .await?;

//     Ok(recs)
// }

pub async fn insert_tag(pool: &DbPool, new_tag: NewTag) -> Result<Option<Tag>> {
    if new_tag.name.is_empty() || new_tag.color.is_empty() {
        return Ok(None);
    }

    let re = Regex::new(r"^#[0-9a-fA-F]{6}$")?;
    if !re.is_match(&new_tag.color) {
        return Ok(None);
    };

    let rec = sqlx::query_as!(
        Tag,
        r#"
            INSERT INTO tags (name, color)
            VALUES ($1, $2)
            RETURNING
                uuid,
                name, color 
        "#,
        new_tag.name,
        new_tag.color
    )
    .fetch_one(pool)
    .await?;
    Ok(Some(rec))
}

// pub async fn update_task(
//     pool: &DbPool,
//     task_uuid: String,
//     updated_task: UpdateTask,
// ) -> Result<bool> {
//     let uuid = Uuid::from_str(&task_uuid)?;
//     let mut any_field = false;

//     let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE tasks SET ");
//     let mut separated = builder.separated(",");

//     if let Some(name) = &updated_task.name {
//         separated.push("name = ").push_bind_unseparated(name);
//         any_field = true;
//     }

//     if let Some(description) = &updated_task.description {
//         separated
//             .push("description = ")
//             .push_bind_unseparated(description);
//         any_field = true;
//     }

//     if let Some(status) = &updated_task.status {
//         separated.push("status = ").push_bind_unseparated(status);
//         any_field = true;
//     }

//     if let Some(priority) = &updated_task.priority {
//         separated
//             .push("priority = ")
//             .push_bind_unseparated(priority);
//         any_field = true;
//     }

//     if let Some(position) = &updated_task.position {
//         separated
//             .push("position = ")
//             .push_bind_unseparated(position);
//         any_field = true;
//     }

//     if !any_field {
//         return Ok(false);
//     }

//     builder.push(" WHERE uuid = ").push_bind(uuid);

//     let query = builder.build();
//     let rows_affected = query.execute(pool).await?.rows_affected();

//     Ok(rows_affected > 0)
// }

// pub async fn delete_task(pool: &DbPool, task_uuid: String) -> Result<bool> {
//     let uuid = Uuid::from_str(&task_uuid)?;
//     let rows_affected = sqlx::query!(
//         r#"
//             DELETE FROM tasks
//             WHERE uuid = $1
//         "#,
//         uuid
//     )
//     .execute(pool)
//     .await?
//     .rows_affected();

//     Ok(rows_affected > 0)
// }
