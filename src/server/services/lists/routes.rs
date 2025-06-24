use actix_web::{
    Scope,
    web::{self},
};

use super::handlers;
use crate::server::services::tasks;

pub fn services() -> Scope {
    web::scope("/lists")
        .route("", web::get().to(handlers::list_lists))
        .route("", web::post().to(handlers::create_list))
        .route(
            "/{uuid}/tasks",
            web::get().to(tasks::handlers::list_tasks_for_list),
        )
        .route("/{uuid}", web::get().to(handlers::get_list))
        .route("/{uuid}", web::put().to(handlers::update_list))
        .route("/{uuid}", web::delete().to(handlers::delete_list))
}
