use crate::entity::admin_user_entity::AdminUser;
use crate::request::admin_auth_request::AdminRegisterRequest;
use crate::vo::admin_users_vo::AdminUsersVo;
use rbatis::rbdc::DateTime;
use rufu_common::bootstrap::database::get_db;
use rufu_common::errors::AppError;
use rufu_common::utils::rand_utils;
use serde_json::{from_value, json};

// 后台用户注册
pub async fn add_admin_user(req: AdminRegisterRequest) -> Result<AdminUsersVo, AppError> {
    let db = get_db()?;

    let admin_user = AdminUser::select_by_column(db, "username", &req.username).await?;
    if !admin_user.is_empty() {
        return Err(AppError::VALIDATE_FIELD_ERROR("用户名已存在".to_string()));
    }

    let sqids = sqids::Sqids::builder().min_length(10).build()?;

    let admin_user = AdminUser {
        id: Some(sqids.encode(&[5, 2, 1])?),
        username: req.username,
        nickname: req.nickname,
        // 生成加密密码
        password: Some(
            rand_utils::hash_password(req.password.unwrap_or("123456".to_string())).await?,
        ),
        avatar: req.avatar,
        sex: req.sex,
        email: req.email.clone(),
        phone: req.phone.clone(),
        status: Some("9".to_string()),
        created_at: Some(DateTime::now().format("YYYY-MM-DD hh:mm:ss")),
        updated_at: Some(DateTime::now().format("YYYY-MM-DD hh:mm:ss")),
    };

    AdminUser::insert(db, &admin_user).await?;

    let admin_users_vo: AdminUsersVo = from_value(json!(admin_user))?;

    Ok(admin_users_vo)
}

// 根据用户ID获取用户信息
pub async fn get_admin_user(user_id: Option<String>) -> Result<AdminUsersVo, AppError> {
    let db = get_db()?;

    let admin_user = AdminUser::select_by_column(db, "id", user_id).await?;

    if admin_user.is_empty() {
        return Err(AppError::RESOURCE_NOT_FOUND("用户信息不存在".to_string()));
    }

    let admin_users_vo: AdminUsersVo = from_value(json!(admin_user.first()))?;

    Ok(admin_users_vo)
}
