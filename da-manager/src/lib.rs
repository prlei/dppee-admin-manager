use once_cell::sync::Lazy;
use rbatis::RBatis;

use crate::entity::config::ApplicationConfig;
use crate::service::sys_dist_service::SysDictService;

pub mod controller;
pub mod entity;
pub mod mapper;
pub mod service;
pub mod errors;

pub static APPLICATION_CONTEXT: Lazy<AppContext> = Lazy::new(|| AppContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::APPLICATION_CONTEXT.rb.clone()
    };
}

pub struct AppContext {
    pub config: ApplicationConfig,
    pub rb: RBatis,
    pub service: InitService,
}

impl Default for AppContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        AppContext {
            rb: init_database(&config),
            service: init_service(),
            config,
        }
    }
}

impl AppContext {
    /// init database pool
    pub async fn init_pool(&self) {
        log::info!(
            "[da_admin] rbatis pool init ({})...",
            self.config.database_url()
        );
        let driver = rbdc_mysql::driver::MysqlDriver {};
        let driver_name = format!("{:?}", driver);
        self.rb
            .init(driver, &self.config.database_url())
            .expect("[abs_admin] rbatis pool init fail!");
        self.rb.acquire().await.expect(&format!(
            "rbatis connect database(driver={},url={}) fail",
            driver_name, &self.config.database_url()
        ));
        log::info!(
            "[da_admin] rbatis pool init success! pool state = {:?}",
            self.rb.get_pool().expect("pool not init!").status()
        );
    }
}

pub struct InitService {
    pub sys_dict_service: SysDictService,
}

pub fn init_database(config: &ApplicationConfig) -> RBatis {
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:rootroot@127.0.0.1:3306/dppee").unwrap();
    return rb;
}

pub fn init_service() -> InitService {
    let service = InitService {
        sys_dict_service: SysDictService {},
    };
    return service;
}


