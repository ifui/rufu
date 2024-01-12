use crate::entity::admin_users_entity::AdminUsers;
use crate::request::admin_auth_request::AdminSignRequest;
use crate::vo::admin_users_vo::AdminUsersVo;
use rufu_common::bootstrap::database::get_db;
use rufu_common::errors::AppError;
use rufu_common::utils::rand_utils::verify_password;
use serde_json::{from_value, json};

/// 用户名登录
pub async fn login_with_username(req: AdminSignRequest) -> Result<AdminUsersVo, AppError> {
    let db = get_db()?;

    let admin_user_vec = AdminUsers::select_by_column(db, "username", &req.username).await?;

    let admin_user = admin_user_vec
        .first()
        .ok_or(AppError::VALIDATE_FIELD_ERROR("用户不存在".to_string()))?;

    // 校验密码
    verify_password(req.password, admin_user.clone().password).await?;

    println!("{:?}", admin_user);

    let admin_users_vo: AdminUsersVo = from_value(json!(admin_user))?;

    Ok(admin_users_vo)
}
