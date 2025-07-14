use super::config::{self, CONFIG};
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};
use anyhow::{Context, Result};
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslVerifyMode};

mod services;

pub async fn run() -> Result<()> {
    let pool = config::init_db_pool().await?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let mut server = HttpServer::new(move || {
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
    });

    server = match CONFIG.ssl.is_valid {
        true => {
            tracing::warn!("HTTPS");
            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder.set_private_key_file(&CONFIG.ssl.key.clone().unwrap(), SslFiletype::PEM)?;
            builder.set_certificate_chain_file(&CONFIG.ssl.crt.clone().unwrap())?;
            builder.set_verify(SslVerifyMode::PEER);
            server.bind_openssl(CONFIG.app.server_url(), builder)
        }
        false => {
            tracing::warn!("HTTP");
            server.bind(CONFIG.app.server_url())
        }
    }
    .context("could not bind server")?;

    Ok(server.run().await?)
}
