#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

mod errors;
mod private;
mod public;

use keygate_core::{config::Environment, Keygate, KeygateConfig};
use poem::listener::TcpListener;
use poem_openapi::LicenseObject;
use tracing::{error, warn};

pub fn license() -> LicenseObject {
    LicenseObject::new("Apache-2.0")
        .identifier("Apache-2.0")
        .url("https://www.apache.org/licenses/LICENSE-2.0.html")
}

pub async fn run(mut config: KeygateConfig) -> color_eyre::Result<()> {
    if config.environment == Environment::Development {
        warn!("\nWARNING: Running in development mode. CORS is enabled for all origins.\n");

        if config.node_id == "__unset__" {
            warn!("WARNING: Node ID is not set. Defaulting to 'development'.");
            config.node_id = "development".to_string();
        }
    }

    if config.environment == Environment::Production && config.node_id == "__unset__" {
        error!("Node ID is not set, but is required in production mode.");
        std::process::exit(1);
    }

    let keygate = Keygate::new(config).await?;

    let private_app = private::PrivateApi::create_app(keygate.clone());
    let public_app = public::PublicApi::create_app(keygate.clone());

    let private_server = poem::Server::new(TcpListener::bind("127.0.0.1:3000")).run(private_app);
    let public_server = poem::Server::new(TcpListener::bind("127.0.0.1:3001")).run(public_app);

    let keygate_tasks = keygate.run();

    tokio::select! {
        res = keygate_tasks => res?,
        res = private_server => res?,
        res = public_server => res?,
    }

    Ok(())
}
