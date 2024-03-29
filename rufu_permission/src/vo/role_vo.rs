use crate::entity::permission_entity::Permission;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleVo {
    pub role_id: Option<u32>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub sort: Option<u32>,
    pub remark: Option<String>,
    pub is_default: Option<String>,
    pub domain: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub permissions: Option<Vec<Permission>>,
}
