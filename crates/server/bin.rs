use keygate_core::{config::ServerConfig, KeygateConfig};
use keygate_server::run;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = KeygateConfig {
        server: ServerConfig {
            keygate_domain: "accounts.localhost".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    run(config).await
}
