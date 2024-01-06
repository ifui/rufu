mod app;
mod routes;

use crate::routes::{admin, api};
use axum::Router;
use rufu_common::bootstrap;
use rufu_common::bootstrap::application::APP_CONFIG;
use rufu_common::middleware::map_response_middleware;

/**
 * 程序入口
 */
#[tokio::main]
async fn main() {
    bootstrap::application::start().await;

    let app = Router::new()
        .nest("/admin", admin::routes())
        .nest("/api", api::routes())
        .layer(axum::middleware::map_response(map_response_middleware));

    let server = format!("{}:{}", APP_CONFIG.host, APP_CONFIG.port);

    let listener = tokio::net::TcpListener::bind(server).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
