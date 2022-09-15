use actix_web::{get, web, App, HttpServer, Responder};
use keysignal_core::{Configuration, KeySignal};
use std::io::Result;

type KS = web::Data<KeySignal>;

pub async fn run() -> Result<()> {
    let keysignal = web::Data::new(KeySignal::new(Configuration {}));

    HttpServer::new(move || {
        let api = web::scope("/identity").service(greet);
        App::new()
            .app_data(keysignal.clone())
            .service(web::scope("/api/v0").service(api))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>, data: KS) -> impl Responder {
    format!("Hello {name}! {:?}", data.get())
}
