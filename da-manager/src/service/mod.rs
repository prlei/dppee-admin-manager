use once_cell::sync::Lazy;
use rbatis::RBatis;
use crate::service::sys_dist_service::SysDictService;

pub mod sys_dist_service;

pub static APPLICATION_CONTEXT22222: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::service::APPLICATION_CONTEXT.rb.clone();
    };
}

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
