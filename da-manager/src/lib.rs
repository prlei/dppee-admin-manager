mod entity;
mod service;
mod controller;

use std::sync::Arc;
use once_cell::sync::Lazy;
use rbatis::RBatis;
use tokio::fs::read_to_string;
use crate::entity::config::ApplicationConfig;
use crate::service::sys_dist_service::SysDictService;


pub static APPLICATION_CONTEXT: Lazy<AppContext> = Lazy::new(|| AppContext::default());

pub struct AppContext {
    pub config: ApplicationConfig,
    pub rb: RBatis,
    pub init_service: InitService,
}

pub struct InitService {
    pub sys_dict_service: SysDictService,
}

impl Default for AppContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        AppContext {
            rb: init_rbatis(&config),
            init_service: init_service(),
            config,
        }
    }
}

pub fn init_rbatis(config: &ApplicationConfig) -> RBatis {
    let rbatis = RBatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 application.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}

pub async fn init_service() -> InitService {
    let service = InitService {
        sys_dict_service: SysDictService {},
    };
    return service;
}

















pub async fn init_context() {
    // 1. 初始化配置信息
    init_config().await;
    // 2. 初始化数据源
    init_database().await;
    // 3. 初始化service信息
    init_service().await;
}

pub async fn init_config() {
    let content = read_to_string("application.yml").await.unwrap();
    let config = ApplicationConfig::new(content.as_str());
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}

pub async fn init_database() {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, &dppee_config.database_url()).unwrap();
    let shared_state = Arc::new(AppState { /* ... */ batis: rb.clone() });
    CONTEXT.set::<Arc<AppState>>(shared_state);
}

pub struct AppState {
    pub batis: RBatis,
}
