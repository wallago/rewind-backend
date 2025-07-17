use actix_web::{
    Scope,
    web::{self},
};

use super::handlers;
use crate::server::services::tags;

pub fn services() -> Scope {
    web::scope("/tasks")
        .route("", web::get().to(handlers::list_tasks))
        .route("", web::post().to(handlers::create_task))
        .route("/{uuid}", web::get().to(handlers::get_task))
        .route("/switch", web::put().to(handlers::switch_tasks))
        .route("/{uuid}", web::put().to(handlers::update_task))
        .route("/{uuid}", web::delete().to(handlers::delete_task))
        .route(
            "/{uuid}/tags",
            web::get().to(tags::handlers::list_tags_for_task),
        )
        .route("/tags", web::post().to(handlers::link_tag_to_task))
}
