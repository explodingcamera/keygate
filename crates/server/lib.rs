#![deny(unsafe_code)]

mod errors;
mod private;
mod public;

use std::net::SocketAddr;

use axum::Router;
use keygate_core::{config::Environment, Keygate, KeygateConfig};
use tracing::{error, info, warn};

pub async fn run(mut config: KeygateConfig) -> color_eyre::Result<()> {
    let now = std::time::Instant::now();

    if config.environment == Environment::Development {
        warn!("Running in development mode. CORS is enabled for all origins.");

        if config.node_id == "__unset__" {
            warn!("Node ID is not set. Defaulting to 'development'.");
            config.node_id = "development".to_string();
        }
    }

    if config.environment == Environment::Production && config.node_id == "__unset__" {
        error!("Node ID is not set, but is required in production mode.");
        std::process::exit(1);
    }

    let keygate = Keygate::new(config).await?;

    let private_app = Router::new().merge(private::new()).with_state(keygate.clone());
    let public_app = Router::new().merge(public::new()).with_state(keygate.clone());

    let private_server = axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(private_app.into_make_service_with_connect_info::<SocketAddr>());

    let public_server = axum::Server::bind(&"127.0.0.1:3001".parse().unwrap())
        .serve(public_app.into_make_service_with_connect_info::<SocketAddr>());

    info!("Keygate started in {}ms", now.elapsed().as_millis());
    info!("Private API listening on {}", "http://localhost:3000");
    info!("Public API listening on {}", "http://localhost:3001");

    let keygate_tasks = keygate.run();

    tokio::select! {
        res = keygate_tasks => res?,
        res = private_server => res?,
        res = public_server => res?,
    };

    Ok(())
}
