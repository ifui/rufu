use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleCategory {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub sort: Option<i32>,
    pub pid: Option<u32>,
}

crud!(ArticleCategory {});
