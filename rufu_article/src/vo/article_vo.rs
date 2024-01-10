use crate::entity::article_category_entity::ArticleCategory;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleVo {
    pub id: Option<u32>,
    pub pre_title: Option<String>,
    pub title: Option<String>,
    pub sub_title: Option<String>,
    pub sort: Option<i32>,
    pub copyfrom: Option<String>,
    pub author: Option<String>,
    pub category_id: Option<u32>,
    pub status: Option<String>,
    pub create_by: Option<String>,
    pub category: Option<ArticleCategory>,
    // article_data
    pub content: Option<String>,
}
