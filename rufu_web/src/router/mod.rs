use crate::openapi::api_router::get_openapi_routes;
use axum::Router;
use rufu_common::middleware::map_response_middleware;

pub mod admin_routes;
pub mod api_routes;

pub fn get_router() -> Router {
    Router::new()
        .merge(admin_routes::routes())
        .merge(api_routes::routes())
        .merge(get_openapi_routes())
        .layer(axum::middleware::map_response(map_response_middleware))
}
