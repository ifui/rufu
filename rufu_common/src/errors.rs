use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum AppError {
    // 服务器错误
    SERVER_ERROR(String),
    // 数据库错误
    RBATIS_ERROR(String),
    // 参数校验错误
    VALIDATE_ERROR(String, Value),
    // 参数字段校验错误
    VALIDATE_FIELD_ERROR(String),
    // JSON序列化错误
    JSON_SERIALIZE,
    // JWT错误
    JWT_ERROR(String),
    // 无权限访问
    UNAUTHORIZED,
}

impl AppError {
    pub fn get_error(&self) -> (i32, String, Option<Value>) {
        match self {
            AppError::SERVER_ERROR(e) => (400, format!("服务器错误: {}", e), None),
            AppError::RBATIS_ERROR(e) => (403, format!("数据库错误: {}", e), None),
            AppError::VALIDATE_ERROR(_, v) => (422, "参数校验错误".to_string(), Some(v.clone())),
            AppError::VALIDATE_FIELD_ERROR(e) => (422, e.to_string(), None),
            AppError::JSON_SERIALIZE => (405, "JSON序列化错误".to_string(), None),
            AppError::JWT_ERROR(e) => (406, e.to_string(), None),
            AppError::UNAUTHORIZED => (401, "对不起，您没有权限访问".to_string(), None),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl From<anyhow::Error> for AppError {
    fn from(arg: anyhow::Error) -> Self {
        AppError::SERVER_ERROR(arg.to_string())
    }
}

impl From<rbatis::rbdc::Error> for AppError {
    fn from(arg: rbatis::rbdc::Error) -> Self {
        AppError::RBATIS_ERROR(arg.to_string())
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(arg: validator::ValidationErrors) -> Self {
        AppError::VALIDATE_ERROR(arg.to_string(), json!(arg.into_errors()))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        AppError::JSON_SERIALIZE
    }
}

impl From<(serde_json::Error,)> for AppError {
    fn from(_: (serde_json::Error,)) -> Self {
        AppError::JSON_SERIALIZE
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(arg: jsonwebtoken::errors::Error) -> Self {
        match arg.kind() {
            ErrorKind::InvalidToken => AppError::JWT_ERROR(arg.to_string()),
            ErrorKind::ExpiredSignature => AppError::JWT_ERROR("用户信息已过期".to_string()),
            _ => AppError::JWT_ERROR(arg.to_string()),
        }
    }
}
