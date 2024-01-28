use crate::request::user_role_request::UserRoleRequest;
use crate::service::user_role_service::{assign_user_roles, remove_user_roles};
use rufu_common::json::RufuJson;
use rufu_common::response::{AppResponse, AppResult};

/// 赋予后台用户角色
#[utoipa::path(post, path = "/admin/user_role", tag = "rufu_permission")]
#[axum::debug_handler]
pub async fn user_role_add_controller(req: RufuJson<UserRoleRequest>) -> AppResult<()> {
    let req = req.validate()?;
    if req.role_ids.is_some() {
        assign_user_roles(req).await?;
    }

    Ok(AppResponse::ok())
}

/// 删除后台用户角色
#[utoipa::path(post, path = "/admin/user_role/delete", tag = "rufu_permission")]
#[axum::debug_handler]
pub async fn user_role_delete_controller(req: RufuJson<UserRoleRequest>) -> AppResult<()> {
    let req = req.validate()?;
    if req.role_ids.is_some() {
        remove_user_roles(req).await?;
    }

    Ok(AppResponse::ok())
}
