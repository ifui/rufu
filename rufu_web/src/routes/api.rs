use crate::app::controller::admin::admin_auth_controller::admin_login;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router {
    auth_routes().merge(un_auth_routes())
}

// 需要权限认证的路由
pub fn auth_routes() -> Router {
    Router::new()
}

// 不需要权限认证的路由
pub fn un_auth_routes() -> Router {
    Router::new().route("/login", post(admin_login))
}
