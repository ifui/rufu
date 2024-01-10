use crate::app::controller::admin::admin_article_category_controller::{
    article_category_add_controller, article_category_delete_controller,
    article_category_list_controller, article_category_update_controller,
};
use crate::app::controller::admin::admin_article_controller::{
    article_add_controller, article_delete_controller, article_list_controller,
    article_show_controller, article_update_controller,
};
use crate::app::controller::admin::admin_auth_controller::{
    admin_login_controller, admin_register_controller, admin_userinfo_controller,
};
use axum::routing::{get, post, put};
use axum::Router;
use rufu_auth::middleware::jwt_middleware;
use rufu_common::middleware::paginate_request_middleware;

pub fn routes() -> Router {
    auth_routes().merge(un_auth_routes())
}

/// 需要权限认证的路由
pub fn auth_routes() -> Router {
    Router::new()
        .route("/userinfo", get(admin_userinfo_controller))
        // 文章
        .route("/article", post(article_add_controller))
        .route("/article/:id", put(article_update_controller))
        .route("/article/:id", get(article_show_controller))
        .route("/article/delete", post(article_delete_controller))
        .route("/article", get(article_list_controller))
        // 文章类别
        .route("/article/category", post(article_category_add_controller))
        .route("/article/category", get(article_category_list_controller))
        .route(
            "/article/category/:id",
            put(article_category_update_controller),
        )
        .route(
            "/article/category/delete",
            post(article_category_delete_controller),
        )
        .layer(axum::middleware::from_fn(jwt_middleware))
        .layer(axum::middleware::from_fn(paginate_request_middleware))
}

/// 不需要权限认证的路由
pub fn un_auth_routes() -> Router {
    Router::new()
        .route("/login", post(admin_login_controller))
        .route("/register", post(admin_register_controller))
        .layer(axum::middleware::from_fn(paginate_request_middleware))
}
