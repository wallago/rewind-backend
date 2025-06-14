use super::handlers;
use actix_web::{
    Scope,
    web::{self},
};

pub fn services() -> Scope {
    web::scope("/lists")
        .route("", web::get().to(handlers::list_lists))
        .route("", web::post().to(handlers::create_list))
    // .route("/{id}", web::get().to(handlers::get_task))
    // .route("/{id}", web::put().to(handlers::update_task))
    // .route("/{id}", web::delete().to(handlers::delete_task))
}
