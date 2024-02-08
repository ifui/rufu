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
use crate::app::controller::admin::admin_permission_controller::{
    admin_permission_add_controller, admin_permission_all_controller,
    admin_permission_assign_by_role_controller, admin_permission_delete_by_role_controller,
    admin_permission_delete_controller, admin_permission_list_controller,
    admin_permission_refresh_controller, admin_permission_update_controller,
};
use crate::app::controller::admin::admin_role_controller::{
    admin_role_add_controller, admin_role_delete_controller, admin_role_list_controller,
    admin_role_show_controller, admin_role_update_controller,
};
use crate::app::controller::admin::admin_user_role_controller::{
    user_role_add_controller, user_role_delete_controller,
};
use crate::middleware::permission_middleware::user_permission_middleware;
use axum::routing::{delete, get, post, put};
use axum::Router;
use rufu_auth::middleware::jwt_middleware;
use rufu_common::middleware::paginate_request_middleware;

pub fn routes() -> Router {
    auth_routes().merge(un_auth_routes())
}

/// 需要权限认证的路由
pub fn auth_routes() -> Router {
    Router::new()
        .route("/admin/userinfo", get(admin_userinfo_controller))
        // 文章
        .route(
            "/admin/article",
            post(article_add_controller).get(article_list_controller),
        )
        .route(
            "/admin/article/:id",
            put(article_update_controller)
                .get(article_show_controller)
                .delete(article_delete_controller),
        )
        // 文章类别
        .route(
            "/admin/article/category",
            post(article_category_add_controller).get(article_category_list_controller),
        )
        .route(
            "/admin/article/category/:id",
            put(article_category_update_controller).delete(article_category_delete_controller),
        )
        // 角色管理
        .route(
            "/admin/role",
            post(admin_role_add_controller)
                .get(admin_role_list_controller)
                .put(admin_role_update_controller),
        )
        .route(
            "/admin/role/:id",
            delete(admin_role_delete_controller).get(admin_role_show_controller),
        )
        // 权限管理
        .route(
            "/admin/permission",
            post(admin_permission_add_controller)
                .get(admin_permission_list_controller)
                .put(admin_permission_update_controller),
        )
        .route(
            "/admin/permission/:id",
            delete(admin_permission_delete_controller),
        )
        // 根据OpenApi更新权限数据
        .route(
            "/admin/permission/refresh",
            post(admin_permission_refresh_controller),
        )
        .route(
            "/admin/permission/all",
            get(admin_permission_all_controller),
        )
        .route(
            "/admin/role_permission",
            post(admin_permission_assign_by_role_controller),
        )
        .route(
            "/admin/role_permission/:role_id/:permission_id",
            delete(admin_permission_delete_by_role_controller),
        )
        // 操作用户角色
        .route("/admin/user_role", post(user_role_add_controller))
        .route("/admin/user_role/delete", post(user_role_delete_controller))
        // 后台用户权限中间件
        .layer(axum::middleware::from_fn(user_permission_middleware))
        .layer(axum::middleware::from_fn(jwt_middleware))
        .layer(axum::middleware::from_fn(paginate_request_middleware))
}

/// 不需要权限认证的路由
pub fn un_auth_routes() -> Router {
    Router::new()
        .route("/admin/login", post(admin_login_controller))
        .route("/admin/register", post(admin_register_controller))
        .layer(axum::middleware::from_fn(paginate_request_middleware))
}
