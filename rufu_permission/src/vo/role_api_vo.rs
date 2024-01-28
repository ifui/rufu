use crate::entity::role_entity::Role;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleApiVo {
    pub id: Option<u32>,
    pub role_id: Option<u32>,
    pub role: Option<Role>,
    pub path: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub method: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
