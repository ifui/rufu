use rbatis::{crud, impl_select};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub role_id: Option<u32>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub sort: Option<u32>,
    pub remark: Option<String>,
    pub is_default: Option<String>,
    pub domain: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

crud!(Role {});

impl_select!(Role {
    select_all_by_sort_order() => "`order by sort`"
});

impl_select!(Role {
    select_default_role_by_domain(domain: Option<String>) => "` where is_default = '1' and domain = #{domain}`"
});
