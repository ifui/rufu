use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct ArticleCategoryRequest {
    #[validate(length(min = 1, message = "长度最少1个字符"))]
    #[validate(required(message = "该项必填"))]
    pub name: Option<String>,
    pub sort: Option<i32>,
    pub pid: Option<u32>,
}
