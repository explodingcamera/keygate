use std::fs;

use utoipa::OpenApi;

fn main() {
    let private_doc = keygate_server::PrivateAPI::openapi()
        .to_pretty_json()
        .expect("failed to serialize openapi document");
    let public_doc = keygate_server::PublicAPI::openapi()
        .to_pretty_json()
        .expect("failed to serialize openapi document");

    fs::write("crates/server/private.json", private_doc).expect("failed to write openapi document");
    fs::write("crates/server/public.json", public_doc).expect("failed to write openapi document");
}
