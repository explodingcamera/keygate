use utoipa::{
    openapi::{
        security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
        OpenApiBuilder,
    },
    OpenApi,
};
use utoipa_swagger_ui::{SwaggerUi, Url};

pub fn public_api_docs() -> utoipa_swagger_ui::SwaggerUi {
    let refresh_token_scheme =
        SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("refresh_token")));

    let access_token_scheme = SecurityScheme::Http(
        HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("BEARER")
            .build(),
    );

    let api = crate::api::public::PublicApiDoc::openapi();

    let mut components = api.components.unwrap();
    components.add_security_scheme("refresh_token", refresh_token_scheme);
    components.add_security_scheme("access_token", access_token_scheme);

    let open_api = OpenApiBuilder::new()
        // .external_docs(Some(ExternalDocsBuilder::new().url("test").build()))
        .components(Some(components))
        .paths(api.paths)
        .tags(api.tags)
        // .info(
        //     InfoBuilder::new()
        //         .license(Some(LicenseBuilder::new().name("pog").build()))
        //         .build(),
        // )
        .build();

    SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
        Url::new("KeyGate Public API", "/docs/openapi-keygate-public.json"),
        open_api,
    )])
}

pub fn admin_api_docs() -> utoipa_swagger_ui::SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
        Url::new("KeyGate Admin API", "/docs/openapi-keygate-admin.json"),
        crate::api::admin::AdminApiDoc::openapi(),
    )])
}
