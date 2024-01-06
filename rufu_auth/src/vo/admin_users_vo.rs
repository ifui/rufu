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
}
