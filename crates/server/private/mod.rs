use keygate_core::Keygate;
use poem::{web::Query, Route};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

pub struct PrivateApi {
    pub keygate: Keygate,
}

impl PrivateApi {
    pub fn create_app(keygate: Keygate) -> Route {
        let service = OpenApiService::new(Self { keygate }, "Keygate Private API", "v0")
            .description("The private API for Keygate, used for backend communication.")
            .license(crate::license())
            .server("/api/private/v0");
        let swagger = service.swagger_ui();

        Route::new().nest("/api/private/v0", service).nest("/openapi", swagger)
    }
}

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
