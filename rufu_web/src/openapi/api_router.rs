use crate::openapi::api_doc::ApiDoc;
use axum::Router;
use rufu_common::bootstrap::application::APP_CONFIG;
use std::ops::Deref;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn get_openapi_routes() -> Router {
    let app_config = APP_CONFIG.deref();
    Router::new().merge(
        SwaggerUi::new(&*app_config.swagger_url).url(&*app_config.openapi_url, ApiDoc::openapi()),
    )
}
