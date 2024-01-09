use crate::app::controller::admin::admin_article_category_controller::{
    article_category_add, article_category_delete, article_category_list, article_category_update,
};
use crate::app::controller::admin::admin_auth_controller::{
    admin_login, admin_register, admin_userinfo,
};
use axum::routing::{delete, get, post, put};
use axum::Router;
use rufu_auth::middleware::jwt_middleware;

pub fn routes() -> Router {
    auth_routes().merge(un_auth_routes())
}

// 需要权限认证的路由
pub fn auth_routes() -> Router {
    Router::new()
        .route("/userinfo", get(admin_userinfo))
        .route("/article/category", post(article_category_add))
        .route("/article/category", get(article_category_list))
        .route("/article/category/:id", put(article_category_update))
        .route("/article/category", delete(article_category_delete))
        .layer(axum::middleware::from_fn(jwt_middleware))
}

// 不需要权限认证的路由
pub fn un_auth_routes() -> Router {
    Router::new()
        .route("/login", post(admin_login))
        .route("/register", post(admin_register))
}
