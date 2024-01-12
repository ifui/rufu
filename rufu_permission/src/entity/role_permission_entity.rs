use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolePermission {
    role_id: u32,
    permission_id: u32,
}
