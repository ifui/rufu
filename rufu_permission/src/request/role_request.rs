use rufu_common::validate::validate_domain::validate_domain;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct RoleRequest {
    #[validate(length(min = 1, message = "长度最少1个字符"))]
    #[validate(required(message = "该项必填"))]
    pub role_name: Option<String>,
    #[validate(length(min = 1, message = "长度最少1个字符"))]
    #[validate(required(message = "该项必填"))]
    pub role_key: Option<String>,
    pub sort: Option<u32>,
    pub remark: Option<String>,
    pub is_default: Option<bool>,
    #[validate(custom = "validate_domain")]
    pub domain: Option<String>,
}
