use keygate_core::KeygateConfig;
use keygate_standalone::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = KeygateConfig {
        storage_type: keygate_core::StorageType::InMemory,
        ..Default::default()
    };

    run(config).await
}
