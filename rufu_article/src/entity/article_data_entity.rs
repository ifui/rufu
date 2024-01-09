use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleData {
    pub article_id: i32,
    pub content: String,
}

crud!(ArticleData {});
