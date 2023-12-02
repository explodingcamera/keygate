#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod errors;
mod middleware;
mod openapi;
mod private;
mod public;

pub use private::PrivateAPI;
pub use public::PublicAPI;

use std::{future::IntoFuture, net::SocketAddr};

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

    let private_app = Router::new()
        .merge(private::new())
        .with_state(keygate.clone());
    let public_app = Router::new()
        .merge(public::new())
        .with_state(keygate.clone());

    let socket = tokio::net::TcpListener::bind(&"127.0.0.1:3000").await?;
    let private_server = axum::serve(
        socket,
        private_app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    let socket = tokio::net::TcpListener::bind(&"127.0.0.1:3001").await?;
    let public_server = axum::serve(
        socket,
        public_app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    let elapsed = now.elapsed();
    let elapsed = if elapsed.as_micros() < 3000 {
        format!("{}ms", elapsed.as_micros() as f64 / 1000.0)
    } else {
        format!("{}ms", elapsed.as_millis())
    };
    info!("Keygate started in {}", elapsed);
    info!("Private API listening on {}", "http://localhost:3000");
    info!("Public API listening on {}", "http://localhost:3001");

    keygate.create_admin_app().await?;
    keygate.create_admin_user().await?;

    let keygate_tasks = keygate.run();

    tokio::select! {
        res = keygate_tasks => res?,
        res = private_server.into_future() => res?,
        res = public_server.into_future() => res?,
    };

    Ok(())
}
