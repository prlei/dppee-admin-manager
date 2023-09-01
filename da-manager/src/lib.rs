use std::sync::Arc;

use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use state::Container;
use tokio::fs::read_to_string;

use crate::entity::config::ApplicationConfig;
use crate::service::ServiceContext;

pub mod controller;
pub mod entity;
pub mod mapper;
pub mod service;
pub mod errors;
pub mod utils;

//----
pub static CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub struct AppState {
    pub batis: RBatis,
}

pub fn init_db() -> RBatis {
    let rb = RBatis::new();
    let config = CONTEXT.get::<ApplicationConfig>();
    rb.init(MysqlDriver {}, config.database_url()).unwrap();
    return rb;
}


#[macro_export]
macro_rules! db_pool {
    () => {
        &mut $crate::init_db().clone()
    };
}

/*初始化环境上下文*/
pub async fn init_context() {
    //第一加载配置
    init_config().await;
    //第二初始化log
    init_log().await;
    //第三初始化数据源
    // init_database().await;
    //第四配置service信息
    init_service().await;
}

//初始化配置信息
pub async fn init_config() {
    let content = read_to_string("application.yml").await.unwrap();
    let config = ApplicationConfig::new(content.as_str());
    CONTEXT.set::<ApplicationConfig>(config);
}

pub async fn init_database() {
    let config = CONTEXT.get::<ApplicationConfig>();
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, config.database_url()).unwrap();
    let shared_state = Arc::new(AppState { /* ... */ batis: rb.clone() });
    CONTEXT.set::<Arc<AppState>>(shared_state);
}

pub async fn get_server() -> String {
    let config = CONTEXT.get::<ApplicationConfig>();
    let server = format!(
        "{}:{}",
        config.server().host(),
        config.server().port()
    );
    return server;
}

pub async fn init_log() {
    // log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

pub async fn init_service() {
    CONTEXT.set::<ServiceContext>(ServiceContext::default());
}

