use actix_web::{
    Scope,
    web::{self},
};

use super::handlers;

pub fn services() -> Scope {
    web::scope("/tasks")
        .route("", web::get().to(handlers::list_tasks))
        .route("", web::post().to(handlers::create_task))
        .route("/{uuid}", web::get().to(handlers::get_task))
        .route("/{uuid}", web::put().to(handlers::update_task))
        .route("/{uuid}", web::delete().to(handlers::delete_task))
}
