use crate::entity::role_entity::Role;
use crate::request::role_request::RoleRequest;
use rbatis::rbdc::DateTime;
use rufu_common::bootstrap::database::get_db;
use rufu_common::errors::AppError;
use rufu_common::utils::mix_utils::ternary_operation;

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
        is_default: ternary_operation(
            req.is_default.eq(&Some(true)),
            Some("1".to_string()),
            Some("0".to_string()),
        ),
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
pub async fn role_update_service(req: RoleRequest, role_id: u32) -> Result<(), AppError> {
    let db = get_db()?;

    let role_key_exists = Role::select_by_column(db, "role_key", &req.role_key).await?;

    for item in role_key_exists {
        if item.role_id != Some(role_id) {
            return Err(AppError::VALIDATE_FIELD_ERROR(
                "role_key 已存在".to_string(),
            ));
        }
    }

    let role = Role {
        role_id: Some(role_id),
        role_name: req.role_name,
        role_key: req.role_key,
        sort: req.sort,
        remark: req.remark,
        is_default: ternary_operation(
            req.is_default.eq(&Some(true)),
            Some("1".to_string()),
            Some("0".to_string()),
        ),
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
