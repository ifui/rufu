use rbatis::{crud, impl_select, impl_select_page};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    pub permission_id: Option<u32>,
    pub path: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub method: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

crud!(Permission {});

impl_select_page!(
    Permission {
       select_list_page(
            permission_id: Option<u32>,
            path: Option<String>,
            method: Option<String>,
            summary: Option<String>,
            tags: Option<String>,
        ) => " ` where 1=1`
              if permission_id != null:
                ` and permission_id = #{permission_id}`
              if path != '':
                ` and path like #{'%'+path+'%'}`
              if method != '':
                ` and method = #{method}`
              if summary != '':
                ` and summary like #{'%'+summary+'%'}`
              if tags != '':
                ` and tags = #{tags}`
              ` order by tags asc`
       "
});

impl_select!(
    Permission {
        select_path_method_column(path: Option<String>, method: Option<String>) => "`where path = #{path} and method = #{method}`"
    }
);
