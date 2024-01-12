use crate::entity::admin_users_entity::AdminUsers;
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

    let admin_user = AdminUsers::select_by_column(db, "username", &req.username).await?;
    if !admin_user.is_empty() {
        return Err(AppError::VALIDATE_FIELD_ERROR("用户名已存在".to_string()));
    }

    let sqids = sqids::Sqids::builder().min_length(10).build()?;

    let admin_user = AdminUsers {
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

    AdminUsers::insert(db, &admin_user).await?;

    let admin_users_vo: AdminUsersVo = from_value(json!(admin_user))?;

    Ok(admin_users_vo)
}
