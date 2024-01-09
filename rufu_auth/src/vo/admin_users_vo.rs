use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminUsersVo {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    pub sex: String,
    pub email: String,
    pub phone: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminUserWithTokenVo {
    pub token: String,
    pub userinfo: AdminUsersVo,
    pub expire: i64,
}
