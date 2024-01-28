use crate::service::user_role_service::get_user_all_permission;
use axum::extract::Request;
use axum::http::Uri;
use axum::middleware::Next;
use axum::response::Response;
use rufu_auth::interface::UserExt;
use rufu_auth::middleware::JwtClaims;
use rufu_common::bootstrap::application::APP_CONFIG;
use rufu_common::errors::AppError;
use rufu_permission::entity::permission_entity::Permission;

// 匹配单个路径
fn match_request_uri(uris: &Vec<&str>, match_path: &str) -> bool {
    let paths: Vec<&str> = match_path.split("/").collect();
    let mut index = 0;

    for uri in uris {
        let _p = String::from(paths[index]);
        index += 1;

        // 完全匹配通配符，则直接返回 true
        if _p.eq("*") {
            return true;
        }
        // 当路径以 : * 开头时，则表示可变路径，进入下一循环
        if (_p.starts_with(":") || _p.starts_with("*")) && !uri.is_empty() {
            continue;
        }
        // 当完全匹配时，进入下一循环
        if _p.eq(uri) {
            continue;
        }
        // 条件都不匹配，返回 false
        return false;
    }

    // 循环结束，没有错误，返回 true
    true
}

// 根据权限列表进行匹配
fn can_authorize(method: &str, uri: &Uri, permissions: Vec<Permission>) -> bool {
    let method = method.to_uppercase();
    let method = method.as_str();

    let uris: Vec<&str> = uri.path().split("/").collect();

    for permission in permissions {
        let p_method = permission.method.unwrap().to_uppercase();
        let p_method = p_method.as_str();
        // 校验请求方法，当为ANY时则表示任意请求方法
        if p_method.eq("ANY") || method.eq(p_method) {
            // 匹配路径，若匹配成功直接返回 true
            if match_request_uri(&uris, permission.path.unwrap().as_str()) {
                return true;
            }
        }
    }
    false
}

pub async fn user_permission_middleware(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let user_ext: Option<&UserExt> = request.extensions().get::<UserExt>();
    let claims: Option<&JwtClaims> = request.extensions().get::<JwtClaims>();

    // 根据 extension 提取用户实例
    // 若无，则返回无权限错误
    if user_ext.is_none() {
        return Err(AppError::UNAUTHORIZED);
    }
    let user_ext = user_ext.unwrap();

    // 判断是否为超级管理员
    let super_admin_ids: &Vec<&str> = &APP_CONFIG.super_admin_id.split(",").collect();
    for super_admin_id in super_admin_ids {
        if user_ext.get_user_id().eq(&super_admin_id.to_string()) {
            let response = next.run(request).await;
            return Ok(response);
        }
    }

    // 获取该用户拥有的权限列表
    let user_permissions =
        get_user_all_permission(user_ext.get_user_id(), claims.cloned().unwrap().domain).await?;
    // 进行鉴权
    let method = request.method().as_str();

    let is_authorize = can_authorize(method, request.uri(), user_permissions);
    if !is_authorize {
        return Err(AppError::UNAUTHORIZED);
    }

    let response = next.run(request).await;
    Ok(response)
}
