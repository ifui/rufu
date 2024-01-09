use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminUsers {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub avatar: String,
    pub sex: String,
    pub email: String,
    pub phone: String,
    pub created_at: String,
    pub updated_at: String,
}

crud!(AdminUsers {});
