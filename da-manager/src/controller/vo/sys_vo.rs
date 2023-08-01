use serde::{Deserialize, Serialize};
use crate::entity::sys_entity::SysUser;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLoginVO {
    pub mobile: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    pub inner: SysUser,
    pub permissions: Vec<String>,
    pub access_token: String,
    // pub role: Option<SysRoleVO>,
}
