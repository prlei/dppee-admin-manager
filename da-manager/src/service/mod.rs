use crate::service::dist_service::SysDictService;
use crate::service::sys_service::SysService;

pub mod dist_service;
pub mod sys_service;

pub struct ServiceContext {
    pub sys_dict_service: SysDictService,
    pub sys_service: SysService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            sys_dict_service: SysDictService {},
            sys_service: SysService {},
        }
    }
}
