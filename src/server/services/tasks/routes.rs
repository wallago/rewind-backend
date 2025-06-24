use super::handlers;
use actix_web::{
    Scope,
    web::{self},
};

pub fn services() -> Scope {
    web::scope("/tasks")
        .route("", web::get().to(handlers::list_tasks))
        .route("", web::post().to(handlers::create_task))
        .route("/{id}", web::get().to(handlers::get_task))
        .route("/{id}", web::put().to(handlers::update_task))
        .route("/{id}", web::delete().to(handlers::delete_task))
}
