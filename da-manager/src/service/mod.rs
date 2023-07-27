use crate::service::sys_dist_service::SysDictService;

pub mod sys_dist_service;

pub struct ServiceContext {
    pub sys_dict_service: SysDictService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            sys_dict_service: SysDictService {},
        }
    }
}
