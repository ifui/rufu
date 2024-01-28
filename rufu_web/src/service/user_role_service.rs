use crate::entity::user_role_entity::UserRole;
use crate::request::user_role_request::UserRoleRequest;
use rufu_common::bootstrap::database::get_db;
use rufu_common::errors::AppError;
use rufu_permission::entity::permission_entity::Permission;
use rufu_permission::entity::role_entity::Role;
use rufu_permission::entity::role_permission_entity::RolePermission;

// 获取用户拥有的权限列表
pub async fn get_user_all_permission(
    user_id: String,
    domain: String,
) -> Result<Vec<Permission>, AppError> {
    let db = get_db()?;

    let user_roles = UserRole::select_by_column(db, "user_id", user_id).await;

    // 根据 domain 查找默认角色
    let default_roles = Role::select_default_role_by_domain(db, Some(domain)).await?;

    return match user_roles {
        Ok(user_roles) => {
            let mut role_ids = vec![];
            // 加入用户拥有角色ID
            for admin_user_role in user_roles {
                role_ids.push(admin_user_role.role_id)
            }
            // 加入默认角色ID
            for default_role in default_roles {
                role_ids.push(default_role.role_id)
            }

            if role_ids.is_empty() {
                return Ok(vec![]);
            }

            let role_permissions =
                RolePermission::select_in_column(db, "role_id", &role_ids).await?;
            let mut permission_ids = vec![];
            // 写入角色权限
            for role_permission in role_permissions {
                permission_ids.push(role_permission.permission_id);
            }

            if permission_ids.is_empty() {
                return Ok(vec![]);
            }

            let admin_user_permissions =
                Permission::select_in_column(db, "permission_id", &permission_ids).await?;

            Ok(admin_user_permissions)
        }
        Err(_) => Ok(vec![]),
    };
}

// 为用户赋予多个角色
pub async fn assign_user_roles(req: UserRoleRequest) -> Result<(), AppError> {
    let role_ids = req.role_ids.unwrap();
    if role_ids.clone().is_empty() {
        return Ok(());
    }

    let db = get_db()?;
    for role_id in role_ids {
        UserRole::replace_into(db, req.user_id.clone(), Some(role_id), req.domain.clone()).await?;
    }

    Ok(())
}

// 删除用户拥有的角色
pub async fn remove_user_roles(req: UserRoleRequest) -> Result<(), AppError> {
    let role_ids = req.role_ids.unwrap();
    if role_ids.clone().is_empty() {
        return Ok(());
    }

    let db = get_db()?;
    UserRole::delete_user_id_in_role_id(db, req.user_id.clone(), role_ids).await?;

    Ok(())
}
