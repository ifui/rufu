use rbatis::{crud, impl_delete, impl_select};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolePermission {
    pub role_id: Option<u32>,
    pub permission_id: Option<u32>,
}

crud!(RolePermission {});

impl_select!(RolePermission {
    select_role_id_permission_id(role_id: Option<u32>, permission_id: Option<u32>) => "`where role_id = #{role_id} and permission_id = #{permission_id}`"
});

impl_select!(RolePermission {
    select_role_ids_has_permission_id(role_ids: Option<Vec<u32>>, permission_id: Option<u32>) =>
        "` where role_id in (`
        trim ',': for _,item in role_ids:
            #{item},
        `) and permission_id = #{permission_id}`
        "
});

impl_delete!(RolePermission {
    delete_by_all_column(role_id: Option<u32>, permission_id: Option<u32>) => "`where role_id = #{role_id} and permission_id = #{permission_id}`"
});

impl_delete!(RolePermission {
    delete_by_permission_ids_column(role_id: Option<u32>, permission_ids: Option<Vec<u32>>) =>
    "`where role_id = #{role_id} and permission_id in (`
    trim ',': for _,item in permission_ids:
        #{item},
    `)`
    "
});
