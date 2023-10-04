use figment::{providers::Env, Figment};
use keygate_core::KeygateConfig;
use keygate_server::run;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    install_tracing();
    color_eyre::install()?;

    let config: KeygateConfig = Figment::new().merge(Env::prefixed("KEYGATE_")).extract()?;
    run(config).await
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::prelude::*;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_filter(LevelFilter::INFO),
        )
        .with(ErrorLayer::default())
        .init();
}
