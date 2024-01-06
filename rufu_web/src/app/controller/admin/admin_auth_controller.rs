use rufu_auth::request::admin_auth_request::{AdminRegisterRequest, AdminSignRequest};
use rufu_auth::service::admin_users_service::add_admin_user;
use rufu_auth::vo::admin_users_vo::AdminUsersVo;
use rufu_common::errors::AppError;
use rufu_common::json::RufuJson;
use rufu_common::response::{AppResponse, AppResult};

// 用户登录
#[axum::debug_handler]
pub async fn admin_login(_: RufuJson<AdminSignRequest>) -> AppResult<String> {
    Err(AppError::SERVER_ERROR("errrrrrr".to_string()))
}

// 后台用户注册
#[axum::debug_handler]
pub async fn admin_register(userinfo: RufuJson<AdminRegisterRequest>) -> AppResult<AdminUsersVo> {
    let res = add_admin_user(userinfo.validate()?).await?;

    Ok(AppResponse::result(res))
}
