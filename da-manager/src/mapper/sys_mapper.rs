use rbatis::{crud, impl_select_page};
use crate::entity::sys_entity::SysUser;

crud!(SysUser {});
impl_select_page!(SysUser{select_page() =>"
     if !sql.contains('count'):
       order by create_date desc"});