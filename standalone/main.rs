mod lib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lib::run().await
}
