#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

mod errors;
mod private;
mod public;

use keygate_core::{config::Environment, Keygate, KeygateConfig};
use poem::{listener::TcpListener, Route};
use poem_openapi::{LicenseObject, OpenApiService};

pub async fn run(config: KeygateConfig) -> eyre::Result<()> {
    if config.environment == Environment::Development {
        println!("\nWARNING: Running in development mode. CORS is enabled for all origins.\n");
    }

    let keygate = Keygate::new(config).await?;

    let private_api = OpenApiService::new(private::PrivateApi, "Keygate Private API", "v0")
        .description("The private API for Keygate, used for backend communication.")
        .license(
            LicenseObject::new("Apache-2.0")
                .identifier("Apache-2.0")
                .url("https://www.apache.org/licenses/LICENSE-2.0.html"),
        )
        .server("/api/private/v0");

    let public_api = OpenApiService::new(public::PublicApi, "Keygate Public API", "v0")
        .description("The public API for Keygate, used for frontend communication.")
        .license(
            LicenseObject::new("Apache-2.0")
                .identifier("Apache-2.0")
                .url("https://www.apache.org/licenses/LICENSE-2.0.html"),
        )
        .server("/api/public/v0");

    let private_api_swagger = private_api.swagger_ui();
    let public_api_swagger = public_api.swagger_ui();

    let private_app = Route::new()
        .nest("/api/private/v0", private_api)
        .nest("/openapi", private_api_swagger);

    let public_app = Route::new()
        .nest("/api/public/v0", public_api)
        .nest("/openapi", public_api_swagger);

    let private_server = poem::Server::new(TcpListener::bind("127.0.0.1:3000")).run(private_app);
    let public_server = poem::Server::new(TcpListener::bind("127.0.0.1:3001")).run(public_app);

    tokio::select! {
        res = private_server => res?,
        res = public_server => res?,
    }

    Ok(())
}
