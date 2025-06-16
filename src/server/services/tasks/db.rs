use anyhow::Result;

use crate::{
    config::DbPool,
    models::tasks::{NewTask, Status, Task},
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

// pub fn get_task_by_uuid(pool: &DbPool, task_uuid: String) -> Result<Task> {
//     let mut conn = pool.get()?;
//     // Ok(tasks::table
//     //     .filter(tasks::dsl::uuid.eq(task_uuid))
//     //     .first::<Task>(&mut *conn)?)
// }

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
