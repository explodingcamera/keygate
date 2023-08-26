use keygate_core::Keygate;
use poem::{web::Query, Route};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

pub struct PublicApi {
    pub keygate: Keygate,
}

impl PublicApi {
    pub fn create_app(keygate: Keygate) -> Route {
        let service = OpenApiService::new(Self { keygate }, "Keygate Public API", "v0")
            .description("The public API for Keygate, used for frontend communication.")
            .license(crate::license())
            .server("/api/public/v0");
        let swagger = service.swagger_ui();

        Route::new().nest("/api/public/v0", service).nest("/openapi", swagger)
    }
}

#[OpenApi]
impl PublicApi {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}
