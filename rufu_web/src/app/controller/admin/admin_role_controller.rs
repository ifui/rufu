use axum::extract::Path;
use axum::Extension;
use rufu_auth::entity::admin_user_entity::AdminUser;
use rufu_common::json::RufuJson;
use rufu_common::response::{AppResponse, AppResult};
use rufu_permission::entity::role_entity::Role;
use rufu_permission::request::role_request::RoleRequest;
use rufu_permission::service::role_service::{
    role_add_service, role_delete_service, role_list_service, role_update_service,
};

/// 新增角色
#[axum::debug_handler]
#[utoipa::path(post, path = "/admin/role", tag = "rufu_permission")]
pub async fn admin_role_add_controller(
    Extension(admin_user): Extension<AdminUser>,
    req: RufuJson<RoleRequest>,
) -> AppResult<()> {
    role_add_service(req.validate()?, admin_user.username.unwrap().to_string()).await?;
    Ok(AppResponse::ok())
}

/// 角色列表
#[axum::debug_handler]
#[utoipa::path(get, path = "/admin/role", tag = "rufu_permission")]
pub async fn admin_role_list_controller() -> AppResult<Vec<Role>> {
    let res = role_list_service().await?;
    Ok(AppResponse::result(res))
}

/// 更新角色
#[axum::debug_handler]
#[utoipa::path(put, path = "/admin/role/:id", tag = "rufu_permission")]
pub async fn admin_role_update_controller(
    Path(id): Path<u32>,
    req: RufuJson<RoleRequest>,
) -> AppResult<()> {
    role_update_service(req.validate()?, id).await?;
    Ok(AppResponse::ok())
}

/// 删除角色
#[axum::debug_handler]
#[utoipa::path(delete, path = "/admin/role/:id", tag = "rufu_permission")]
pub async fn admin_role_delete_controller(Path(id): Path<u32>) -> AppResult<()> {
    role_delete_service(id).await?;
    Ok(AppResponse::ok())
}
