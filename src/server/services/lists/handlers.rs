use super::db;
use crate::{config::DbPool, models::lists::NewList};
use actix_web::{HttpResponse, Responder, web};

pub async fn list_lists(pool: web::Data<DbPool>) -> impl Responder {
    match db::list_all_lists(&pool).await {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// pub async fn get_task(pool: web::Data<DbPool>, task_uuid: web::Path<String>) -> impl Responder {
//     match db::get_task_by_uuid(&pool, task_uuid.into_inner()) {
//         Ok(task) => HttpResponse::Ok().json(task),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

pub async fn create_list(pool: web::Data<DbPool>, new_list: web::Json<NewList>) -> impl Responder {
    match db::insert_list(&pool, new_list.into_inner()).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// pub async fn update_task(
//     pool: web::Data<DbPool>,
//     task_uuid: web::Path<String>,
//     updated_task: web::Json<UpdateTask>,
// ) -> impl Responder {
//     match db::update_task(&pool, task_uuid.into_inner(), updated_task.into_inner()) {
//         Ok(task) => HttpResponse::Ok().json(task),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

// pub async fn delete_task(pool: web::Data<DbPool>, task_uuid: web::Path<String>) -> impl Responder {
//     match db::delete_task(&pool, task_uuid.into_inner()) {
//         Ok(affected) if affected > 0 => HttpResponse::NoContent().finish(),
//         Ok(_) => HttpResponse::NotFound().body("Task not found"),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
