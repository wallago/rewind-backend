use super::db;
use crate::{config::DbPool, models::boards::NewBoard};
use actix_web::{HttpResponse, Responder, web};

pub async fn list_boards(pool: web::Data<DbPool>) -> impl Responder {
    match db::list_all_boards(&pool).await {
        Ok(boards) => HttpResponse::Ok().json(boards),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// pub async fn get_task(pool: web::Data<DbPool>, task_uuid: web::Path<String>) -> impl Responder {
//     match db::get_task_by_uuid(&pool, task_uuid.into_inner()) {
//         Ok(task) => HttpResponse::Ok().json(task),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

pub async fn create_board(
    pool: web::Data<DbPool>,
    new_board: web::Json<NewBoard>,
) -> impl Responder {
    match db::insert_board(&pool, new_board.into_inner()).await {
        Ok(board) => HttpResponse::Ok().json(board),
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
