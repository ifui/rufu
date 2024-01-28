use crate::interface::UserExt;
use axum::extract::Request;
use axum::http::{header, HeaderMap};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, DecodingKey, Validation};
use rufu_common::bootstrap::application::APP_CONFIG;
use rufu_common::errors::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub username: String,
    pub user_id: String,
    pub domain: String,
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
    println!("{:?}", token);

    let claims = decode::<JwtClaims>(
        &token,
        &DecodingKey::from_secret(APP_CONFIG.jwt_secret.as_ref()),
        &Validation::default(),
    )?
    .claims;

    let user_ext = UserExt::new(claims.clone().user_id, claims.clone().domain);

    request.extensions_mut().insert(claims);
    request.extensions_mut().insert(user_ext);
    let response = next.run(request).await;
    Ok(response)
}
