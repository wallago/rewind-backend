use super::db;
use crate::{
    config::DbPool,
    models::boards::{NewBoard, UpdateBoard},
};
use actix_web::{HttpResponse, Responder, web};

pub async fn list_boards(pool: web::Data<DbPool>) -> impl Responder {
    match db::get_all_boards(&pool).await {
        Ok(boards) => HttpResponse::Ok().json(boards),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_board(pool: web::Data<DbPool>, board_uuid: web::Path<String>) -> impl Responder {
    match db::get_board_by_uuid(&pool, board_uuid.into_inner()).await {
        Ok(board) => HttpResponse::Ok().json(board),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_board(
    pool: web::Data<DbPool>,
    new_board: web::Json<NewBoard>,
) -> impl Responder {
    match db::insert_board(&pool, new_board.into_inner()).await {
        Ok(Some(board)) => HttpResponse::Ok().json(board),
        Ok(None) => HttpResponse::BadRequest().body("Wrong values"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_board(
    pool: web::Data<DbPool>,
    board_uuid: web::Path<String>,
    updated_board: web::Json<UpdateBoard>,
) -> impl Responder {
    match db::update_board(&pool, board_uuid.into_inner(), updated_board.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("Board not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_board(
    pool: web::Data<DbPool>,
    board_uuid: web::Path<String>,
) -> impl Responder {
    match db::delete_board(&pool, board_uuid.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("Board not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn switch_boards(
    pool: web::Data<DbPool>,
    boards_uuid: web::Json<(String, String)>,
) -> impl Responder {
    let (board_uuid_from, board_uuid_to) = boards_uuid.into_inner();
    match db::switch_boards_position(&pool, board_uuid_from, board_uuid_to).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("Board not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
