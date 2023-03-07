use std::io::Result;
fn main() -> Result<()> {
    tonic_build::configure()
        .build_client(cfg!(feature = "client"))
        .build_server(cfg!(feature = "server"))
        .compile(
            &[
                "src/ac.proto",
                "src/admin.proto",
                "src/identity.proto",
                "src/meta.proto",
                "src/process.proto",
                "src/session.proto",
                "src/models/models.proto",
            ],
            &["src/", "src/models/"],
        )?;
    Ok(())
}
