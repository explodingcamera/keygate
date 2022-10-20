use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

pub fn public_api_docs() -> utoipa_swagger_ui::SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
        Url::new("api1", "/docs/openapi-keygate-public.json"),
        crate::api::admin::AdminApiDoc::openapi(),
    )])
}

pub fn admin_api_docs() -> utoipa_swagger_ui::SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
        Url::new("api1", "/docs/openapi-keygate-admin.json"),
        crate::api::admin::AdminApiDoc::openapi(),
    )])
}
