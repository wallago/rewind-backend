use super::config::{self, CONFIG};
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};
use anyhow::{Context, Result};
use env_logger::Env;

mod services;

pub async fn run() -> Result<()> {
    let pool = config::init_db_pool().await?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("api")
                    .service(services::boards::routes::services())
                    .service(services::lists::routes::services())
                    .service(services::tasks::routes::services())
                    .service(services::tags::routes::services()),
            )
    })
    .bind(&CONFIG.app.server_url())
    .context("could not bind server")?;
    Ok(server.run().await?)
}
