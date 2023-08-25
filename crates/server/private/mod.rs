use poem::web::Query;
use poem_openapi::{payload::PlainText, OpenApi};

pub struct PrivateApi;

#[OpenApi]
impl PrivateApi {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}
