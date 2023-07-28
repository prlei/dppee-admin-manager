use rbatis::RBatis;

use crate::service::sys_dist_service::SysDictService;

pub mod sys_dist_service;

pub struct ServiceContext {
    pub rb: RBatis,
    pub sys_dict_service: SysDictService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            rb: Default::default(),
            sys_dict_service: SysDictService {},
        }
    }
}
