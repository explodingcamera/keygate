use std::fs;

use utoipa::OpenApi;

fn main() {
    let private_doc = keygate_server::PrivateAPI::openapi()
        .to_json()
        .expect("failed to serialize openapi document");

    let public_doc = keygate_server::PublicAPI::openapi()
        .to_json()
        .expect("failed to serialize openapi document");

    fs::write("api/private.json", private_doc.clone()).expect("failed to write openapi document");
    fs::write("api/public.json", public_doc.clone()).expect("failed to write openapi document");

    fs::write("api/private.ts", to_js(&private_doc)).expect("failed to write openapi document");
    fs::write("api/public.ts", to_js(&public_doc)).expect("failed to write openapi document");
}

fn to_js(json: &str) -> String {
    format!("export default {} as const;", json)
}
