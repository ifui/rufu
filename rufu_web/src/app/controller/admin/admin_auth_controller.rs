use axum::Extension;
use jsonwebtoken::{encode, EncodingKey, Header};
use rufu_auth::entity::admin_users_entity::AdminUsers;
use rufu_auth::middleware::JwtClaims;
use rufu_auth::request::admin_auth_request::{AdminRegisterRequest, AdminSignRequest};
use rufu_auth::service::admin_users_service::add_admin_user;
use rufu_auth::service::auth_service::login_with_username;
use rufu_auth::vo::admin_users_vo::{AdminUserWithTokenVo, AdminUsersVo};
use rufu_common::bootstrap::application::APP_CONFIG;
use rufu_common::json::RufuJson;
use rufu_common::response::{AppResponse, AppResult};

/// 用户登录
#[axum::debug_handler]
pub async fn admin_login(req: RufuJson<AdminSignRequest>) -> AppResult<AdminUserWithTokenVo> {
    let res = login_with_username(req.validate()?).await?;

    // 生成 token
    let jwt_expire = APP_CONFIG.jwt_expire;
    let jwt_secret = APP_CONFIG.jwt_secret.as_ref();
    let exp = (chrono::Utc::now() + chrono::Duration::minutes(jwt_expire)).timestamp();
    let claims: JwtClaims = JwtClaims {
        username: res.username.clone(),
        user_id: res.id.clone(),
        exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret),
    )?;

    let response = AdminUserWithTokenVo {
        token,
        userinfo: res,
        expire: exp,
    };

    Ok(AppResponse::result(response))
}

/// 后台用户注册
#[axum::debug_handler]
pub async fn admin_register(userinfo: RufuJson<AdminRegisterRequest>) -> AppResult<AdminUsersVo> {
    let res = add_admin_user(userinfo.validate()?).await?;

    Ok(AppResponse::result(res))
}

/// 登录用户信息
#[axum::debug_handler]
pub async fn admin_userinfo(Extension(admin_user): Extension<AdminUsers>) -> AppResult<AdminUsers> {
    Ok(AppResponse::result(admin_user))
}
