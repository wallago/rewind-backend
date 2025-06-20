use super::handlers;
use actix_web::{
    Scope,
    web::{self},
};

pub fn services() -> Scope {
    web::scope("/boards")
        .route("", web::get().to(handlers::list_boards))
        .route("", web::post().to(handlers::create_board))
        .route(
            "/{uuid}/lists",
            web::get().to(handlers::list_lists_for_board),
        )
    // .route("/{id}", web::get().to(handlers::get_task))
    // .route("/{id}", web::put().to(handlers::update_task))
    // .route("/{id}", web::delete().to(handlers::delete_task))
}
