use crate::errors::AppError;
use axum::Json;
use serde::{Deserialize, Serialize};

/// 响应模板
#[derive(Debug, Serialize, Deserialize)]
pub struct AppResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> AppResponse<T> {
    pub fn new(code: i32, msg: String, data: Option<T>) -> Json<Self> {
        Json(Self { code, msg, data })
    }

    pub fn ok(msg: String) -> Json<Self> {
        Self::new(0, msg, None)
    }

    pub fn result(data: T) -> Json<Self> {
        Self::new(0, "ok".to_string(), Some(data))
    }

    pub fn error(code: i32, msg: String) -> Json<Self> {
        Self::new(code, msg, None)
    }
}

pub type AppResult<T = i8> = Result<Json<AppResponse<T>>, AppError>;
