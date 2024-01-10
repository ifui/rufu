use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleData {
    pub article_id: Option<u32>,
    pub content: Option<String>,
}

crud!(ArticleData {});
