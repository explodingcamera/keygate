use actix_web::{web, App, HttpServer};
use keysignal_core::{Configuration, KeySignal};
use std::io::Result;
type KS = web::Data<KeySignal>;
mod api;

pub async fn run() -> Result<()> {
    let keysignal = web::Data::new(KeySignal::new(Configuration {}));

    HttpServer::new(move || {
        let api = web::scope("/admin").service(api::admin::get());

        App::new()
            .app_data(keysignal.clone())
            .service(web::scope("/api/v0").service(api))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
