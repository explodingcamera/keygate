#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

use actix_web::{middleware::Logger, web, App, HttpServer};
use keygate_core::{Keygate, KeygateConfig};

mod api;
mod errors;
mod schema;
mod swagger;
mod utils;

type KG = web::Data<Keygate>;
pub async fn run(config: KeygateConfig) -> eyre::Result<()> {
    let keygate_public = web::Data::new(Keygate::new(config.clone()).await?);
    let keygate_admin = keygate_public.clone();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let json_cfg_public = web::JsonConfig::default()
        .limit(2048)
        .error_handler(|err, _| errors::KeygateResponseError::BadRequest(err.to_string()).into());

    let json_cfg_admin = json_cfg_public.clone();

    let admin_api = HttpServer::new(move || {
        let api = web::scope("/admin").service(api::admin::get());

        let admin_service = match &config.server.admin_prefix.clone() {
            Some(prefix) => web::scope(&(prefix.to_owned() + "/api")).service(api),
            None => web::scope("/api").service(api),
        };

        App::new()
            .app_data(json_cfg_public.clone())
            .app_data(keygate_admin.clone())
            .service(admin_service)
            .service(swagger::admin_api_docs())
            .wrap(Logger::new(
                "ADMIN: %a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
    })
    .bind((config.server.admin_interface, config.server.admin_port))?
    .run();

    let public_api = HttpServer::new(move || {
        let api = web::scope("/public").service(api::public::get());

        let public_service = match &config.server.public_prefix {
            Some(prefix) => web::scope(&(prefix.to_owned() + "/api")).service(api),
            None => web::scope("/api").service(api),
        };

        App::new()
            .app_data(json_cfg_admin.clone())
            .app_data(keygate_public.clone())
            .service(public_service)
            .service(swagger::public_api_docs())
            .wrap(Logger::new(
                "PUBLIC: %a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
    })
    .bind((config.server.public_interface, config.server.public_port))?
    .run();

    tokio::select! {
        _ = tokio::spawn(async move {
            public_api.await.unwrap();
        }), if config.server.public_port != 0 => {}
        _ = tokio::spawn(async move {
            admin_api.await.unwrap();
        }), if config.server.admin_port != 0 => {}
    };

    Ok(())
}
