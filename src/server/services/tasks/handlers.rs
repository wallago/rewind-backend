use super::db;
use crate::{
    config::DbPool,
    models::tasks::{LinkTagToTask, NewTask, UpdateTask},
};
use actix_web::{HttpResponse, Responder, web};

pub async fn list_tasks(pool: web::Data<DbPool>) -> impl Responder {
    match db::get_all_tasks(&pool).await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_task(pool: web::Data<DbPool>, task_uuid: web::Path<String>) -> impl Responder {
    match db::get_task_by_uuid(&pool, task_uuid.into_inner()).await {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_task(pool: web::Data<DbPool>, new_task: web::Json<NewTask>) -> impl Responder {
    match db::insert_task(&pool, new_task.into_inner()).await {
        Ok(Some(task)) => HttpResponse::Ok().json(task),
        Ok(None) => HttpResponse::BadRequest().body("Wrong values"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_task(
    pool: web::Data<DbPool>,
    task_uuid: web::Path<String>,
    updated_task: web::Json<UpdateTask>,
) -> impl Responder {
    match db::update_task(&pool, task_uuid.into_inner(), updated_task.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("Task not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_task(pool: web::Data<DbPool>, task_uuid: web::Path<String>) -> impl Responder {
    match db::delete_task(&pool, task_uuid.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("Task not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_tasks_for_list(
    pool: web::Data<DbPool>,
    list_uuid: web::Path<String>,
) -> impl Responder {
    match db::get_tasks_by_list_uuid(&pool, list_uuid.into_inner()).await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn switch_tasks(
    pool: web::Data<DbPool>,
    tasks_uuid: web::Json<(String, String)>,
) -> impl Responder {
    let (task_uuid_from, task_uuid_to) = tasks_uuid.into_inner();
    match db::switch_tasks_position(&pool, task_uuid_from, task_uuid_to).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("Task not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn link_tag_to_task(
    pool: web::Data<DbPool>,
    link: web::Json<LinkTagToTask>,
) -> impl Responder {
    match db::insert_task_tag(&pool, link.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("Task or/and Tag not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
