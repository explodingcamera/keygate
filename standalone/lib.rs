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

    let keygate = web::Data::new(Keygate::new(config));

    HttpServer::new(move || {
        let api = web::scope("/admin").service(api::admin::get());

        App::new()
            .app_data(keygate.clone())
            .service(web::scope("/api/v0").service(api))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
