use crate::openapi::api_doc::ApiDoc;
use axum::extract::Path;
use axum::Extension;
use rufu_common::json::RufuJson;
use rufu_common::query::RufuQuery;
use rufu_common::request::paginate_request::PaginateRequest;
use rufu_common::response::{AppResponse, AppResult, PageData};
use rufu_permission::entity::permission_entity::Permission;
use rufu_permission::request::permission_request::{PermissionQueryRequest, PermissionRequest};
use rufu_permission::request::role_permission_request::RolePermissionRequest;
use rufu_permission::service::permission_service::{
    permission_add_service, permission_assign_by_role_id_service,
    permission_delete_by_role_id_service, permission_delete_service, permission_list_service,
    permission_refresh_by_openapi_service,
};
use utoipa::OpenApi;

/// 权限列表
#[axum::debug_handler]
#[utoipa::path(get, path = "/admin/permission", tag = "rufu_permission")]
pub async fn admin_permission_list_controller(
    Extension(paginate): Extension<PaginateRequest>,
    RufuQuery(req): RufuQuery<PermissionQueryRequest>,
) -> AppResult<PageData<Vec<Permission>>> {
    let res = permission_list_service(req, paginate).await?;

    Ok(AppResponse::result(res))
}

/// 添加权限
#[axum::debug_handler]
#[utoipa::path(post, path = "/admin/permission", tag = "rufu_permission")]
pub async fn admin_permission_add_controller(req: RufuJson<PermissionRequest>) -> AppResult<()> {
    permission_add_service(req.validate()?).await?;

    Ok(AppResponse::ok())
}

/// 删除权限
#[axum::debug_handler]
#[utoipa::path(delete, path = "/admin/permission/:id", tag = "rufu_permission")]
pub async fn admin_permission_delete_controller(id: Path<u32>) -> AppResult<()> {
    permission_delete_service(Some(id.0)).await?;

    Ok(AppResponse::ok())
}

/// 更新权限数据
#[axum::debug_handler]
#[utoipa::path(post, path = "/admin/permission/refresh", tag = "rufu_permission")]
pub async fn admin_permission_refresh_controller() -> AppResult<Vec<Permission>> {
    let openapi = ApiDoc::openapi();

    permission_refresh_by_openapi_service(openapi).await?;

    Ok(AppResponse::ok())
}

/// 给角色分配权限
#[axum::debug_handler]
#[utoipa::path(post, path = "/admin/role_permission", tag = "rufu_permission")]
pub async fn admin_permission_assign_by_role_controller(
    req: RufuJson<RolePermissionRequest>,
) -> AppResult<Vec<Permission>> {
    let req = req.validate()?;
    permission_assign_by_role_id_service(req.role_id, req.permission_id).await?;

    Ok(AppResponse::ok())
}

/// 删除角色分配的权限
#[axum::debug_handler]
#[utoipa::path(
    delete,
    path = "/admin/role_permission/:role_id/:permission_id",
    tag = "rufu_permission"
)]
pub async fn admin_permission_delete_by_role_controller(
    req: Path<(u32, u32)>,
) -> AppResult<Vec<Permission>> {
    let (role_id, permission_id) = req.0;
    permission_delete_by_role_id_service(role_id, permission_id).await?;

    Ok(AppResponse::ok())
}
