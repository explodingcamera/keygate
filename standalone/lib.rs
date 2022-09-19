use actix_web::{get, web, App, HttpServer, Responder};
use keysignal_core::{Configuration, KeySignal};
use std::io::Result;

type KS = web::Data<KeySignal>;

pub async fn run() -> Result<()> {
    let keysignal = web::Data::new(KeySignal::new(Configuration {}));

    HttpServer::new(move || {
        let api = web::scope("/identity").service(test).service(test2);
        App::new()
            .app_data(keysignal.clone())
            .service(web::scope("/api/v0").service(api))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/test2")]
async fn test2() -> impl Responder {
    "Hello!"
}

#[get("/test")]
async fn test(ks: KS) -> impl Responder {
    "Hello"
}
