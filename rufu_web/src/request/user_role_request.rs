use rufu_common::validate::validate_domain::validate_domain;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct UserRoleRequest {
    #[validate(required(message = "该项必填"))]
    pub user_id: Option<String>,
    pub role_ids: Option<Vec<u32>>,
    #[validate(custom = "validate_domain")]
    pub domain: Option<String>,
}
