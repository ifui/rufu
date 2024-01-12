use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub role_id: Option<u32>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub sort: Option<u32>,
    pub remark: Option<String>,
    pub status: Option<String>,
    pub create_by: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
