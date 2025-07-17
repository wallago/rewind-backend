use super::db;
use crate::{config::DbPool, models::tags::NewTag};
use actix_web::{HttpResponse, Responder, web};

pub async fn list_tags(pool: web::Data<DbPool>) -> impl Responder {
    match db::get_all_tags(&pool).await {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_tag(pool: web::Data<DbPool>, tag_uuid: web::Path<String>) -> impl Responder {
    match db::get_tag_by_uuid(&pool, tag_uuid.into_inner()).await {
        Ok(tag) => HttpResponse::Ok().json(tag),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_tag(pool: web::Data<DbPool>, new_tag: web::Json<NewTag>) -> impl Responder {
    match db::insert_tag(&pool, new_tag.into_inner()).await {
        Ok(Some(tag)) => HttpResponse::Ok().json(tag),
        Ok(None) => HttpResponse::BadRequest().body("Wrong values"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_tags_for_task(
    pool: web::Data<DbPool>,
    task_uuid: web::Path<String>,
) -> impl Responder {
    match db::get_tags_by_task_uuid(&pool, task_uuid.into_inner()).await {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_tags_for_board(
    pool: web::Data<DbPool>,
    board_uuid: web::Path<String>,
) -> impl Responder {
    match db::get_tags_by_board_uuid(&pool, board_uuid.into_inner()).await {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// pub async fn update_task(
//     pool: web::Data<DbPool>,
//     task_uuid: web::Path<String>,
//     updated_task: web::Json<UpdateTask>,
// ) -> impl Responder {
//     match db::update_task(&pool, task_uuid.into_inner(), updated_task.into_inner()).await {
//         Ok(true) => HttpResponse::NoContent().finish(),
//         Ok(false) => HttpResponse::NotFound().body("Task not found"),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// pub async fn delete_task(pool: web::Data<DbPool>, task_uuid: web::Path<String>) -> impl Responder {
//     match db::delete_task(&pool, task_uuid.into_inner()).await {
//         Ok(true) => HttpResponse::NoContent().finish(),
//         Ok(false) => HttpResponse::NotFound().body("Task not found"),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// pub async fn list_tasks_for_list(
//     pool: web::Data<DbPool>,
//     list_uuid: web::Path<String>,
// ) -> impl Responder {
//     match db::get_tasks_by_list_uuid(&pool, list_uuid.into_inner()).await {
//         Ok(tasks) => HttpResponse::Ok().json(tasks),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }
