use rufu_auth::request::admin_auth_request::{AdminRegisterRequest, AdminSignRequest};
use rufu_auth::service::admin_users_service::add_admin_user;
use rufu_auth::service::auth_service::login_with_username;
use rufu_auth::vo::admin_users_vo::AdminUsersVo;
use rufu_common::json::RufuJson;
use rufu_common::response::{AppResponse, AppResult};

// 用户登录
#[axum::debug_handler]
pub async fn admin_login(req: RufuJson<AdminSignRequest>) -> AppResult<AdminUsersVo> {
    let res = login_with_username(req.validate()?).await?;

    Ok(AppResponse::result(res))
}

// 后台用户注册
#[axum::debug_handler]
pub async fn admin_register(userinfo: RufuJson<AdminRegisterRequest>) -> AppResult<AdminUsersVo> {
    let res = add_admin_user(userinfo.validate()?).await?;

    Ok(AppResponse::result(res))
}
