use crate::entity::permission_entity::Permission;
use crate::entity::role_entity::Role;
use crate::entity::role_permission_entity::RolePermission;
use crate::request::role_request::RoleRequest;
use crate::vo::role_vo::RoleVo;
use rbatis::rbdc::DateTime;
use rufu_common::bootstrap::database::get_db;
use rufu_common::errors::AppError;
use serde_json::{from_value, json};

/// 添加角色
pub async fn role_add_service(req: RoleRequest, created_by: String) -> Result<(), AppError> {
    let db = get_db()?;

    let role_key_exists = Role::select_by_column(db, "role_key", req.role_key.clone()).await?;
    if !role_key_exists.is_empty() {
        return Err(AppError::VALIDATE_FIELD_ERROR(
            "role_key: 已存在".to_string(),
        ));
    }

    let role = Role {
        role_id: None,
        role_name: req.role_name,
        role_key: req.role_key,
        sort: req.sort.or(Some(0)),
        remark: req.remark.or(Some("".to_string())),
        is_default: req.is_default,
        domain: req.domain,
        created_by: Some(created_by),
        created_at: Some(DateTime::now().format("YYYY-MM-DD hh:mm:ss")),
        updated_at: Some(DateTime::now().format("YYYY-MM-DD hh:mm:ss")),
    };

    Role::insert(db, &role).await?;

    Ok(())
}

/// 角色列表
pub async fn role_list_service() -> Result<Vec<Role>, AppError> {
    let db = get_db()?;
    let roles = Role::select_all_by_sort_order(db).await?;

    Ok(roles)
}

/// 更新角色
pub async fn role_update_service(req: RoleRequest) -> Result<(), AppError> {
    let db = get_db()?;

    let role_key_exists = Role::select_by_column(db, "role_key", &req.role_key).await?;

    for item in role_key_exists {
        if item.role_id != req.role_id {
            return Err(AppError::VALIDATE_FIELD_ERROR(
                "role_key 已存在".to_string(),
            ));
        }
    }

    let role = Role {
        role_id: req.role_id,
        role_name: req.role_name,
        role_key: req.role_key,
        sort: req.sort,
        remark: req.remark,
        is_default: req.is_default,
        domain: req.domain,
        created_by: None,
        created_at: None,
        updated_at: Some(DateTime::now().format("YYYY-MM-DD hh:mm:ss")),
    };

    Role::update_by_column(db, &role, "role_id").await?;

    Ok(())
}

/// 删除角色
pub async fn role_delete_service(role_id: u32) -> Result<(), AppError> {
    let db = get_db()?;
    Role::delete_by_column(db, "role_id", role_id).await?;

    Ok(())
}

/// 角色详情
pub async fn role_show_service(role_id: u32) -> Result<RoleVo, AppError> {
    let db = get_db()?;
    let role = Role::select_by_column(db, "role_id", role_id).await?;
    let role = role
        .first()
        .ok_or(AppError::RBATIS_ERROR("角色不存在".to_string()))?;

    // 查找当前角色拥有的权限
    let role_permissions =
        RolePermission::select_by_column(db, "role_id", role.clone().role_id).await?;
    let permission_ids: Vec<Option<u32>> = role_permissions
        .iter()
        .map(move |e| e.permission_id)
        .collect();

    let mut role_vo: RoleVo = from_value(json!(role))?;

    if permission_ids.is_empty() {
        role_vo.permissions = Some(vec![]);
    } else {
        // 根据权限列表获取权限信息
        let permissions =
            Permission::select_in_column(db, "permission_id", &permission_ids).await?;
        role_vo.permissions = Some(permissions);
    }

    Ok(role_vo)
}
