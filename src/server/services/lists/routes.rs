use super::handlers;
use actix_web::{
    Scope,
    web::{self},
};

pub fn services() -> Scope {
    web::scope("/lists")
        .route("", web::get().to(handlers::list_lists))
        .route("", web::post().to(handlers::create_list))
        .route(
            "/{uuid}/tasks",
            web::get().to(handlers::list_tasks_for_list),
        )
        .route("/{id}", web::get().to(handlers::get_list))
        .route("/{id}", web::put().to(handlers::update_list))
        .route("/{id}", web::delete().to(handlers::delete_list))
}
