use axum::extract::Path;
use axum::Extension;
use rufu_auth::interface::UserExt;
use rufu_common::json::RufuJson;
use rufu_common::response::{AppResponse, AppResult};
use rufu_permission::entity::role_entity::Role;
use rufu_permission::request::role_request::RoleRequest;
use rufu_permission::service::role_service::{
    role_add_service, role_delete_service, role_list_service, role_show_service,
    role_update_service,
};
use rufu_permission::vo::role_vo::RoleVo;

/// 新增角色
#[axum::debug_handler]
#[utoipa::path(post, path = "/admin/role", tag = "rufu_permission")]
pub async fn admin_role_add_controller(
    Extension(user_ext): Extension<UserExt>,
    req: RufuJson<RoleRequest>,
) -> AppResult<()> {
    role_add_service(req.validate()?, user_ext.get_user_id()).await?;
    Ok(AppResponse::ok())
}

/// 角色列表
#[axum::debug_handler]
#[utoipa::path(get, path = "/admin/role", tag = "rufu_permission")]
pub async fn admin_role_list_controller() -> AppResult<Vec<Role>> {
    let res = role_list_service().await?;
    Ok(AppResponse::result(res))
}

/// 角色详情
#[axum::debug_handler]
#[utoipa::path(get, path = "/admin/role/:id", tag = "rufu_permission")]
pub async fn admin_role_show_controller(Path(role_id): Path<u32>) -> AppResult<RoleVo> {
    let res = role_show_service(role_id).await?;
    Ok(AppResponse::result(res))
}

/// 更新角色
#[axum::debug_handler]
#[utoipa::path(put, path = "/admin/role", tag = "rufu_permission")]
pub async fn admin_role_update_controller(req: RufuJson<RoleRequest>) -> AppResult<()> {
    role_update_service(req.validate()?).await?;
    Ok(AppResponse::ok())
}

/// 删除角色
#[axum::debug_handler]
#[utoipa::path(delete, path = "/admin/role/:id", tag = "rufu_permission")]
pub async fn admin_role_delete_controller(Path(id): Path<u32>) -> AppResult<()> {
    role_delete_service(id).await?;
    Ok(AppResponse::ok())
}
