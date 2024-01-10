use rbatis::{crud, impl_select_page};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
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
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

crud!(Article {});

impl_select_page!(Article {select_article_list(
    title: Option<String>,
    author: Option<String>,
    copyfrom: Option<String>,
    id: Option<u32>
) => "`where status = 9`
      if title != '':
        ` and title like #{'%'+title+'%'}`
      if author != '':
        ` and author like #{author}`
      if copyfrom != '':
        ` and copyfrom like #{copyfrom}`
      if id != null:
        ` and id = #{id}`" });
