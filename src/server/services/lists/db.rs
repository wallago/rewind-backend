use anyhow::Result;

use crate::{
    config::DbPool,
    models::lists::{List, NewList},
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

// pub fn get_task_by_uuid(pool: &DbPool, task_uuid: String) -> Result<Task> {
//     let mut conn = pool.get()?;
//     // Ok(tasks::table
//     //     .filter(tasks::dsl::uuid.eq(task_uuid))
//     //     .first::<Task>(&mut *conn)?)
// }

pub async fn insert_list(pool: &DbPool, new_list: NewList) -> Result<List> {
    let rec = sqlx::query_as::<_, List>(
        r#"
            INSERT INTO lists (board_uuid, name, description)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(new_list.board_uuid)
    .bind(new_list.name)
    .bind(new_list.description)
    .fetch_one(pool)
    .await?;
    Ok(rec)
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
