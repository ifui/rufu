use crate::app::controller::admin::admin_auth_controller::{
    admin_login, admin_register, admin_userinfo,
};
use axum::routing::{get, post};
use axum::Router;
use rufu_auth::middleware::jwt_middleware;

pub fn routes() -> Router {
    auth_routes().merge(un_auth_routes())
}

// 需要权限认证的路由
pub fn auth_routes() -> Router {
    Router::new()
        .route("/userinfo", get(admin_userinfo))
        .layer(axum::middleware::from_fn(jwt_middleware))
}

// 不需要权限认证的路由
pub fn un_auth_routes() -> Router {
    Router::new()
        .route("/login", post(admin_login))
        .route("/register", post(admin_register))
}
