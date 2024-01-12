use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminUsers {
    pub id: Option<String>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub avatar: Option<String>,
    pub sex: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

crud!(AdminUsers {});
