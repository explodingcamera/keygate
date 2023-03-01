#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

mod errors;
mod services;

use keygate_core::{config::Environment, KeygateConfig, KeygateSecrets};
use proto::v1::api::identity::identity_service_server::IdentityServiceServer;
use tonic::transport::Server;

use services::identity::IdentityServiceImpl;

pub async fn run(config: KeygateConfig, secrets: KeygateSecrets) -> eyre::Result<()> {
    if config.environment == Environment::Development {
        println!("\nWARNING: Running in development mode. CORS is enabled for all origins.\n");
    }

    let addr = "[::1]:50051".parse().unwrap();
    let service = IdentityServiceImpl::default();

    let gprc_server = Server::builder().add_service(IdentityServiceServer::new(service));

    tokio::select! {
        _ = gprc_server.serve(addr) => {
            println!("gRPC server stopped");
        }
    };

    Ok(())
}
