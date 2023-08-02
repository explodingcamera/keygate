use keygate_core::{config::ServerConfig, KeygateConfig};
use keygate_server::run;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = KeygateConfig {
        server: ServerConfig {
            keygate_domain: "accounts.localhost".to_string(),
            ..Default::default()
        },
        environment: keygate_core::config::Environment::Development,
        storage_options: keygate_core::config::StorageOptions::Sqlite {
            database_path: "db.sql".into(),
        },
    };

    run(config).await
}
