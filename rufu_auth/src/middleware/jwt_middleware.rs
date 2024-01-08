use crate::entity::admin_users_entity::AdminUsers;
use axum::extract::Request;
use axum::http::{header, HeaderMap};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use rufu_common::bootstrap::application::APP_CONFIG;
use rufu_common::bootstrap::database::get_db;
use rufu_common::errors::AppError;
use rufu_common::response::AppResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub user_id: String,
    pub exp: i64,
}

pub async fn jwt_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 获取请求头中的 token
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });
    let token = token.ok_or_else(|| AppError::UNAUTHORIZED)?;

    let claims = decode::<JwtClaims>(
        &token,
        &DecodingKey::from_secret(APP_CONFIG.jwt_secret.as_ref()),
        &Validation::default(),
    )?
    .claims;
    let db = get_db()?;
    let user = AdminUsers::select_by_column(db, "id", &claims.user_id).await?;

    if user.is_empty() {
        return Ok(
            AppResponse::<usize>::error(401, "对不起，该用户没有权限访问".to_string())
                .into_response(),
        );
    }
    let user = user.first().ok_or(AppError::UNAUTHORIZED)?;

    request.extensions_mut().insert(user.to_owned());
    let response = next.run(request).await;
    Ok(response)
}
