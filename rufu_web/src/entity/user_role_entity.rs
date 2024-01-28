use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{crud, pysql, Error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: Option<String>,
    pub role_id: Option<u32>,
    // 所属域
    pub domain: Option<String>,
}

crud!(UserRole {});

impl UserRole {
    pysql!(replace_into(rb: &dyn Executor, user_id: Option<String>, role_id: Option<u32>, domain: Option<String>) -> Result<ExecResult, Error> =>
    "`replace into user_role(user_id, role_id, domain) values (#{user_id}, #{role_id}, #{domain})`" );

    pysql!(delete_user_id_in_role_id(rb: &dyn Executor, user_id: Option<String>, role_ids: Vec<u32>) -> Result<ExecResult, Error> =>
    "`delete from user_role where user_id = #{user_id} and role_id in (`
        trim ',': for _,item in role_ids:
            #{item},
    `)`" );
}
