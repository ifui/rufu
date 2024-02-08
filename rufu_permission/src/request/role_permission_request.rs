use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct RolePermissionRequest {
    #[validate(required(message = "该项必填"))]
    pub role_id: Option<u32>,
    #[validate(required(message = "该项必填"))]
    pub permission_id: Option<Vec<u32>>,
}
