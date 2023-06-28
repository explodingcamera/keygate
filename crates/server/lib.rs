#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

mod errors;
mod services;

use keygate_core::{config::Environment, Keygate, KeygateConfig};

pub async fn run(config: KeygateConfig) -> eyre::Result<()> {
    if config.environment == Environment::Development {
        println!("\nWARNING: Running in development mode. CORS is enabled for all origins.\n");
    }

    let keygate = Keygate::new(config).await?;

    // let addr = "[::1]:50051".parse().unwrap();

    // let gprc_server = Server::builder()
    //     .add_service(keygate.identity.service())
    //     .add_service(keygate.identity.service());

    // tokio::select! {
    //     _ = gprc_server.serve(addr) => {
    //         println!("gRPC server stopped");
    //     }
    // };

    Ok(())
}
