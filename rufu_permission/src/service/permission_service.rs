use crate::entity::permission_entity::Permission;
use crate::entity::role_entity::Role;
use crate::entity::role_permission_entity::RolePermission;
use crate::request::permission_request::{PermissionQueryRequest, PermissionRequest};
use rbatis::PageRequest;
use rufu_common::bootstrap::database::get_db;
use rufu_common::errors::AppError;
use rufu_common::request::paginate_request::PaginateRequest;
use rufu_common::response::PageData;
use rufu_common::utils::date_utils::get_now_date;
use utoipa::openapi::{OpenApi, PathItemType};

/// 基于 OpenApi 更新权限表
pub async fn permission_refresh_by_openapi_service(openapi: OpenApi) -> Result<(), AppError> {
    let db = get_db()?;

    let mut permissions: Vec<Permission> = vec![];

    for (url, item) in openapi.paths.paths {
        for (path_item_type, operation) in item.operations {
            let method;
            match path_item_type {
                PathItemType::Get => method = "get",
                PathItemType::Post => method = "post",
                PathItemType::Put => method = "put",
                PathItemType::Delete => method = "delete",
                PathItemType::Options => method = "options",
                PathItemType::Head => method = "head",
                PathItemType::Patch => method = "patch",
                PathItemType::Trace => method = "trace",
                PathItemType::Connect => method = "connect",
            }
            let p = Permission {
                permission_id: None,
                path: Some(url.to_string()),
                summary: operation.summary,
                description: operation.description,
                tags: Some(operation.tags.unwrap().join(",")),
                method: Some(method.to_string()),
                created_at: Some(get_now_date()),
                updated_at: Some(get_now_date()),
            };
            permissions.push(p);
        }
    }

    // 循环更新
    for mut permission in permissions {
        let exist_permission = Permission::select_path_method_column(
            db,
            permission.clone().path,
            permission.clone().method,
        )
        .await?;
        let first_permission = exist_permission.first();
        match first_permission {
            None => Permission::insert(db, &permission).await?,
            Some(e) => {
                permission.permission_id = e.permission_id;
                Permission::update_by_column(db, &permission, "permission_id").await?
            }
        };
    }

    Ok(())
}

// 添加权限
pub async fn permission_add_service(req: PermissionRequest) -> Result<(), AppError> {
    let db = get_db()?;

    let method = req.clone().method.unwrap().to_lowercase();

    // 判断是否有重复的权限
    let permissions =
        Permission::select_path_method_column(db, req.clone().path, Some(method.clone())).await?;
    if !permissions.is_empty() {
        return Err(AppError::VALIDATE_FIELD_ERROR("该权限已存在".to_string()));
    }

    let permission = Permission {
        permission_id: None,
        path: req.path,
        summary: req.summary,
        description: req.description,
        tags: req.tags,
        method: Some(method),
        created_at: Some(get_now_date()),
        updated_at: Some(get_now_date()),
    };
    Permission::insert(db, &permission).await?;

    Ok(())
}

// 权限列表
pub async fn permission_list_service(
    req: PermissionQueryRequest,
    paginate: PaginateRequest,
) -> Result<PageData<Vec<Permission>>, AppError> {
    let db = get_db()?;

    let page_request = PageRequest::new(paginate.page, paginate.page_size);

    let permissions = Permission::select_list_page(
        db,
        &page_request,
        req.permission_id,
        req.path,
        req.method,
        req.summary,
        req.tags,
    )
    .await?;

    let res = PageData {
        total: permissions.total,
        page: permissions.page_no,
        page_size: permissions.page_size,
        records: Some(permissions.records),
    };

    Ok(res)
}

// 删除权限
pub async fn permission_delete_service(permission_id: Option<u32>) -> Result<(), AppError> {
    let db = get_db()?;

    Permission::delete_by_column(db, "permission_id", permission_id).await?;

    Ok(())
}

// 给角色分配权限
pub async fn permission_assign_by_role_id_service(
    role_id: Option<u32>,
    permission_id: Option<u32>,
) -> Result<(), AppError> {
    let db = get_db()?;

    let exist_role_permission =
        RolePermission::select_role_id_permission_id(db, role_id, permission_id).await?;

    match exist_role_permission.first() {
        None => {
            let role_permission = RolePermission {
                role_id,
                permission_id,
            };

            let role = Role::select_by_column(db, "role_id", role_id).await?;
            if role.is_empty() {
                return Err(AppError::VALIDATE_FIELD_ERROR("角色不存在".to_string()));
            }
            let permission =
                Permission::select_by_column(db, "permission_id", permission_id).await?;
            if permission.is_empty() {
                return Err(AppError::VALIDATE_FIELD_ERROR("权限不存在".to_string()));
            }

            RolePermission::insert(db, &role_permission).await?;
        }
        Some(_) => return Ok(()),
    }

    Ok(())
}

// 删除角色分配的权限
pub async fn permission_delete_by_role_id_service(
    role_id: u32,
    permission_id: u32,
) -> Result<(), AppError> {
    let db = get_db()?;

    RolePermission::deleteby_by_all_column(db, Some(role_id), Some(permission_id)).await?;

    Ok(())
}

// 判断角色ID是否拥有该权限
pub async fn role_has_permission(role_id: Vec<u32>, permission_id: u32) -> Result<(), AppError> {
    let db = get_db()?;

    let role_permission =
        RolePermission::select_role_ids_has_permission_id(db, Some(role_id), Some(permission_id))
            .await?;

    if role_permission.is_empty() {
        return Err(AppError::UNAUTHORIZED);
    }

    Ok(())
}
