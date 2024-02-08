use crate::openapi::api_router::get_openapi_routes;
use axum::Router;
use rufu_common::middleware::map_response_middleware;
use tower_http::cors::CorsLayer;

pub mod admin_routes;
pub mod api_routes;

pub fn get_router() -> Router {
    let cors = CorsLayer::permissive();
    Router::new()
        .merge(admin_routes::routes())
        .merge(api_routes::routes())
        .merge(get_openapi_routes())
        .layer(axum::middleware::map_response(map_response_middleware))
        .layer(cors)
}
