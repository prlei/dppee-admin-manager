use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUser {
    pub id: Option<i32>,
    pub gmt_create: Option<DateTime>,
    pub gmt_modified: Option<DateTime>,
    pub username: Option<String>,
    pub gender: Option<i32>,
    pub email: Option<String>,
    pub status_id: Option<i32>,
    pub sort: Option<i32>,
    pub user_no: Option<usize>,
    pub mobile: Option<String>,
    pub real_name: Option<String>,
    pub remark: Option<String>,
    pub password: Option<String>,
}

/**
 *struct:SysUserQuery
 *desc:用户列表查询参数
 *author:String
 *email:348040933
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysUserQuery {
    pub id: i64,
    pub username: Option<String>,
    pub gender: Option<String>,
    pub mail: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub order: Option<String>,
    pub order_field: Option<String>,
}