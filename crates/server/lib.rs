#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

use keygate_core::{config::Environment, KeygateConfig, KeygateSecrets};

mod errors;

pub async fn run(config: KeygateConfig, secrets: KeygateSecrets) -> eyre::Result<()> {
    if config.environment == Environment::Development {
        println!("\nWARNING: Running in development mode. CORS is enabled for all origins.\n");
    }

    // tokio::select! {
    //     _ = tokio::spawn(async move {
    //         public_api.await.unwrap();
    //     }), if config.server.public_port != 0 => {}
    //     _ = tokio::spawn(async move {
    //         admin_api.await.unwrap();
    //     }), if config.server.admin_port != 0 => {}
    // };

    Ok(())
}
