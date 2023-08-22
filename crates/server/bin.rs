use figment::{providers::Env, Figment};
use keygate_core::KeygateConfig;
use keygate_server::run;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config: KeygateConfig = Figment::new().merge(Env::prefixed("KG_")).extract()?;

    run(config).await
}
