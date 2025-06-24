use super::db;
use crate::{
    config::DbPool,
    models::lists::{NewList, UpdateList},
};
use actix_web::{HttpResponse, Responder, web};

pub async fn list_lists(pool: web::Data<DbPool>) -> impl Responder {
    match db::list_all_lists(&pool).await {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_list(pool: web::Data<DbPool>, list_uuid: web::Path<String>) -> impl Responder {
    match db::get_list_by_uuid(&pool, list_uuid.into_inner()).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_list(pool: web::Data<DbPool>, new_list: web::Json<NewList>) -> impl Responder {
    match db::insert_list(&pool, new_list.into_inner()).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_tasks_for_list(
    pool: web::Data<DbPool>,
    list_uuid: web::Path<String>,
) -> impl Responder {
    match db::list_all_tasks_for_list(&pool, list_uuid.into_inner()).await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_list(
    pool: web::Data<DbPool>,
    list_uuid: web::Path<String>,
    updated_list: web::Json<UpdateList>,
) -> impl Responder {
    match db::update_list(&pool, list_uuid.into_inner(), updated_list.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("List not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_list(pool: web::Data<DbPool>, list_uuid: web::Path<String>) -> impl Responder {
    match db::delete_list(&pool, list_uuid.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("List not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
