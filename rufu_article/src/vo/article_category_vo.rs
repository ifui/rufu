use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleCategoryVo {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub sort: Option<i32>,
    pub pid: Option<u32>,
    pub children: Vec<ArticleCategoryVo>,
}
