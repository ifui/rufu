mod app;
mod entity;
mod middleware;
mod openapi;
mod request;
mod router;
mod service;
mod traits;

use crate::router::get_router;
use rufu_common::bootstrap;
use rufu_common::bootstrap::application::APP_CONFIG;

/**
 * 程序入口
 */
#[tokio::main]
async fn main() {
    bootstrap::application::start().await;

    let router = get_router();

    let server = format!("{}:{}", APP_CONFIG.host, APP_CONFIG.port);

    let listener = tokio::net::TcpListener::bind(server).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
