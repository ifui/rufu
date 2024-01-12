use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminUsersVo {
    pub id: Option<String>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub sex: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminUserWithTokenVo {
    pub token: Option<String>,
    pub userinfo: Option<AdminUsersVo>,
    pub expire: Option<i64>,
}
