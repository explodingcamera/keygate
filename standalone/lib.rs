use actix_web::{web, App, HttpServer};
use keygate_core::{Configuration, Keygate};
use std::io::Result;
type KS = web::Data<Keygate>;
mod api;

pub async fn run() -> Result<()> {
    let config = Configuration {
        storage_type: keygate_core::StorageType::InMemory,
        ..Default::default()
    };

    let keygate_public = web::Data::new(Keygate::new(config.clone()));
    let keygate_admin = keygate_public.clone();

    let admin_api = HttpServer::new(move || {
        let api = web::scope("/admin").service(api::admin::get());

        let admin_service = match &config.admin_prefix.clone() {
            Some(prefix) => web::scope(&(prefix.to_owned() + "/api")).service(api),
            None => web::scope("/api").service(api),
        };

        App::new()
            .app_data(keygate_admin.clone())
            .service(admin_service)
    })
    .bind((config.admin_interface, config.admin_port))?
    .run();

    let public_api = HttpServer::new(move || {
        let api = web::scope("/public").service(api::public::get());

        let public_service = match &config.public_prefix {
            Some(prefix) => web::scope(&(prefix.to_owned() + "/api")).service(api),
            None => web::scope("/api").service(api),
        };

        App::new()
            .app_data(keygate_public.clone())
            .service(public_service)
    })
    .bind((config.public_interface, config.public_port))?
    .run();

    tokio::select! {
        _ = tokio::spawn(async move {
            public_api.await.unwrap();
        }), if config.public_port != 0 => {}
        _ = tokio::spawn(async move {
            admin_api.await.unwrap();
        }), if config.admin_port != 0 => {}
    };

    Ok(())
}
