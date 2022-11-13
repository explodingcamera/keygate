use keygate_core::KeygateConfig;
use keygate_standalone::run;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    let config = KeygateConfig {
        storage_type: keygate_core::StorageType::RocksDB,
        ..Default::default()
    };

    run(config).await
}
