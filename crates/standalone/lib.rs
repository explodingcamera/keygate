#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

use actix_web::{middleware::Logger, web, App, HttpServer};
use keygate_core::{config::Environment, Keygate, KeygateConfig, KeygateSecrets};

mod api;
mod errors;
mod swagger;
mod utils;

type KG = web::Data<Keygate>;
pub async fn run(config: KeygateConfig, secrets: KeygateSecrets) -> eyre::Result<()> {
    if config.environment == Environment::Development {
        println!("\nWARNING: Running in development mode. CORS is enabled for all origins.\n");
    }

    let keygate_public = web::Data::new(Keygate::new(config.clone(), secrets).await?);
    let keygate_admin = keygate_public.clone();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let json_cfg_public = web::JsonConfig::default()
        .limit(2048)
        .error_handler(|err, _| errors::KeygateResponseError::BadRequest(err.to_string()).into());

    let json_cfg_admin = json_cfg_public.clone();

    let admin_api = HttpServer::new(move || {
        let api = api::admin::service("");

        let admin_service = match &config.server.admin_prefix.clone() {
            Some(prefix) => web::scope(&(prefix.to_owned() + "/api/v1")).service(api),
            None => web::scope("/api/v1").service(api),
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
    .bind((config.server.admin_interface.clone(), config.server.admin_port))?
    .run();

    let public_api = HttpServer::new(move || {
        let api = api::public::service("", keygate_public.clone());

        let public_service = match &config.server.public_prefix {
            Some(prefix) => web::scope(&(prefix.to_owned() + "/api/v1")).service(api),
            None => web::scope("/api/v1").service(api),
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
    .bind((config.server.public_interface.clone(), config.server.public_port))?
    .run();

    println!();

    println!("Starting public API on port {}", config.server.public_port);
    println!(
        "api docs: http://{}:{}/api-doc/",
        config.server.public_interface.clone(),
        config.server.public_port
    );

    println!();

    println!("Starting admin API on port {}", config.server.admin_port);
    println!(
        "api docs: http://{}:{}/api-doc/",
        config.server.admin_interface.clone(),
        config.server.admin_port
    );

    println!();

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
