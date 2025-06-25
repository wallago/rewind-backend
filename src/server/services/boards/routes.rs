use actix_web::{
    Scope,
    web::{self},
};

use super::handlers;
use crate::server::services::lists;

pub fn services() -> Scope {
    web::scope("/boards")
        .route("", web::get().to(handlers::list_boards))
        .route("", web::post().to(handlers::create_board))
        .route(
            "/{uuid}/lists",
            web::get().to(lists::handlers::list_lists_for_board),
        )
        .route("/{uuid}", web::get().to(handlers::get_board))
        .route("/switch", web::put().to(handlers::switch_boards))
        .route("/{uuid}", web::put().to(handlers::update_board))
        .route("/{uuid}", web::delete().to(handlers::delete_board))
}
