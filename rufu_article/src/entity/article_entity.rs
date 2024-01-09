use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub pre_title: String,
    pub title: String,
    pub sub_title: String,
    pub sort: i32,
    pub copyfrom: String,
    pub author: String,
    pub category_id: i32,
    pub status: String,
    pub create_by: String,
    pub created_at: String,
    pub updated_at: String,
}

crud!(Article {});
