use std::io::Result;
fn main() -> Result<()> {
    tonic_build::configure()
        .build_client(cfg!(feature = "client"))
        .build_server(cfg!(feature = "server"))
        .compile(&["src/identity.proto", "src/models.proto"], &["src/"])?;

    Ok(())
}
