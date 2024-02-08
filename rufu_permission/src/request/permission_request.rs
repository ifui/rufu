use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct PermissionRequest {
    pub permission_id: Option<u32>,
    #[validate(required(message = "该项必填"))]
    pub path: Option<String>,
    #[validate(required(message = "该项必填"))]
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    #[validate(required(message = "该项必填"))]
    pub method: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct PermissionQueryRequest {
    pub permission_id: Option<u32>,
    pub path: Option<String>,
    pub summary: Option<String>,
    pub method: Option<String>,
    pub tags: Option<String>,
}
