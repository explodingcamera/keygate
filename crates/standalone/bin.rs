use keygate_core::{config::ServerConfig, generate_ed25519_key_pair, KeygateConfig};
use keygate_standalone::run;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    let config = KeygateConfig {
        storage_type: keygate_core::StorageType::RocksDB,
        server: ServerConfig {
            keygate_domain: "accounts.localhost".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let secrets = keygate_core::KeygateSecrets {
        jwt_ed25519_keypair: generate_ed25519_key_pair(),
    };

    run(config, secrets).await
}
