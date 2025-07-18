use actix_web::{
    Scope,
    web::{self},
};

use super::handlers;

pub fn services() -> Scope {
    web::scope("/tags")
        .route("", web::get().to(handlers::list_tags))
        .route("", web::post().to(handlers::create_tag))
        .route("/{uuid}", web::get().to(handlers::get_tag))
        .route("/{uuid}", web::put().to(handlers::update_tag))
        .route("/{uuid}", web::delete().to(handlers::delete_tag))
}
