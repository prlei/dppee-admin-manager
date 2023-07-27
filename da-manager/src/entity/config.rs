use getset::{Getters, MutGetters, Setters};
use tokio::fs::read_to_string;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone, Getters, Setters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ServerConfig {
    ///当前服务地址
    host: String,
    port: String,
}

///服务启动配置
#[derive(
Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone, Getters, Setters, MutGetters,
)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ApplicationConfig {
    //server 配置
    server: ServerConfig,
    /// 数据库地址
    database_url: String,

}

impl ApplicationConfig {
    pub fn new(yml_data: &str) -> Self {
        let config = match serde_yaml::from_str(yml_data) {
            Ok(e) => e,
            Err(e) => panic!("{}", e),
        };
        config
    }
    pub fn validate(&self) {
        if self.database_url.is_empty() {
            panic!("请配置database_url ！！！！！！！！！！！！！！！！！！！")
        }
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let content = read_to_string("application.yml");
        let result = ApplicationConfig::new(content.as_str());
        if result.debug {
            println!("[abs_admin] load config:{:?}", result);
            println!("[abs_admin] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[abs_admin] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}
