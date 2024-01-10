use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn validate_status(value: &str) -> Result<(), ValidationError> {
    // 0 草稿，-1 退回，9 发布
    if value.eq("0") || value.eq("-1") || value.eq("9") {
        return Ok(());
    }

    Err(ValidationError::new("状态参数只允许0、9"))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct ArticleRequest {
    pub pre_title: Option<String>,
    #[validate(required(message = "该项必填"))]
    pub title: Option<String>,
    pub sub_title: Option<String>,
    pub sort: Option<i32>,
    pub copyfrom: Option<String>,
    pub author: Option<String>,
    #[validate(required(message = "该项必填"))]
    pub category_id: Option<u32>,
    #[validate(custom = "validate_status")]
    pub status: Option<String>,
    // article_data
    pub content: Option<String>,
}

/// 检索
#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct ArticleListRequest {
    pub id: Option<u32>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub copyfrom: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    pub page: u64,
    pub page_size: u64,
}
