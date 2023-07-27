use std::string::String;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use once_cell::sync::Lazy;
use rbatis::{crud, impl_select, RBatis};
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use rbdc_mysql::driver::MysqlDriver;
use serde_json::json;
use tokio::fs::read_to_string;

use crate::{AppState, CONTEXT};
use crate::controller::vo::dict_vo::DictPageVO;
use crate::entity::config::ApplicationConfig;
use crate::entity::sys_entity::{DictEditDTO, SysDict, SysUser, SysUserQuery};
use crate::service::{APPLICATION_CONTEXT, ServiceContext};
use crate::service::sys_dist_service::SysDictService;

pub static RB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

crud!(SysUser{});
impl_select!(SysUser{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});

// curl localhost:8080/api/user_save -X POST -d '{"mobile": "bb", "password": "apple"}' --header "Content-Type: application/json"
pub async fn user_list(Json(payload): Json<SysUser>) -> impl IntoResponse {


    // 使用相对路径读取 app.yaml 内容为字符串
    let content = read_to_string("application.yml").await.unwrap();
    // serde_yaml 解析字符串为 User 对象
    let config: ApplicationConfig = ApplicationConfig::new(content.as_str());

    let state = CONTEXT.get::<AppState>();

    println!("config ---->  {:?}", &state.batis);


    log::info!("Sysuser : {:?}", payload);
    let user = SysUser{
        id: Option::from(12),
        gmt_create: Option::from(DateTime::now()),
        gmt_modified: Option::from(DateTime::now()),
        username: Option::from(String::from("john")),
        gender: Option::from(1),
        email: Option::from(String::from("john@example.com")),
        status_id:Option::from(16),
        sort: Option::from(1),
        user_no: Option::from(17),
        mobile: Option::from(String::from("lisa")),
        real_name: Option::from(String::from("ad")),
        remark: Option::from(String::from("ae")),
        password: Option::from(String::from("af")),
    };

    return Json(json!({"mobile": user.mobile, "password": user.password}));
}



pub async fn query_dict_page(Json(item): Json<DictPageVO>) -> impl IntoResponse {
    // let sys_dict = SysDict::select_page(&mut rb, &PageRequest::new(item.page_no.unwrap_or(1), item.page_size.unwrap_or(10))).await;
    // let sys_dict =  APPLICATION_CONTEXT.sys_dict_service.page(&item).await;

    let service_context = CONTEXT.get::<ServiceContext>();
    let sys_dict = service_context.sys_dict_service.page(&item).await;

    return Json(sys_dict);
}






//
// // curl localhost:8080/api/query_user_by_id/1
// pub async fn query_user_by_id(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
//     let mut rb = &state.batis;
//     let user_result = SysUser::select_by_id(&mut rb, id).await;
//     // println!(">>>>> user_result = {:?}", user_result);
//     return Json(user_result);
// }
//
// // curl localhost:8080/api/user_list -X POST -d '{"id":1}' --header "Content-Type: application/json"
// pub async fn user_list(State(state): State<Arc<AppState>>, Json(arg): Json<SysUserQuery>) -> impl IntoResponse {
//     let user_arg = arg;
//     let mut rb = &state.batis;
//     let user_result = SysUser::select_by_id(&mut rb, user_arg.id.to_string()).await;
//     let count: u64 = rb
//         .query_decode("select count(1) as count from sys_user", vec![])
//         .await
//         .unwrap();
//
//     println!(">>>>> count = {}", count);
//     println!(">>>>> user_result = {:?}", user_result);
//     return Json(user_result);
// }
//
//
// // dict methods
//
// // curl localhost:8080/api/query_dict_page -X POST -d '{"page_no":1, "page_size":3}' --header "Content-Type: application/json"
// pub async fn query_dict_page(State(state): State<Arc<AppState>>, Json(item): Json<DictPageVO>) -> impl IntoResponse {
//     let mut rb = &state.batis;
//     let sys_dict = SysDict::select_page(&mut rb, &PageRequest::new(item.page_no.unwrap_or(1), item.page_size.unwrap_or(10))).await;
//     // let sys_dict =  APPLICATION_CONTEXT.sys_dict_service.page(&item).await;
//     return Json(sys_dict);
// }
//
// // curl localhost:8080/api/query_dict_by_id/1
// pub async fn query_dict_by_id(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
//     let mut rb = &state.batis;
//     let dict_result = SysDict::select_by_id(&mut rb, id).await;
//     return Json(dict_result);
// }
//
// // curl localhost:8080/api/query_dict_by_id/1
// pub async fn query_dict(State(state): State<Arc<AppState>>, Json(item): Json<SysDict>) -> impl IntoResponse {
//     log::info!("dict params: {:?}, {:?}", &item, state.batis);
//     let mut rb = &state.batis;
//     let dict_result = SysDict::select_by_column(&mut rb, "code", &item.code).await;
//     return Json(dict_result);
// }
//
// // curl localhost:8080/api/update_dict -X POST -d '{"id":"3", "code":"picipici"}' --header "Content-Type: application/json"
// pub async fn update_dict(State(state): State<Arc<AppState>>, item: Json<DictEditDTO>) -> impl IntoResponse {
//     let mut rb = &state.batis;
//     let data = SysDict::from(&item.0);
//     let dict_result = SysDict::update_by_column(&mut rb, &data,"id").await;
//     return Json(dict_result);
// }
//
// pub async fn add_dict(State(state): State<Arc<AppState>>, Json(item): Json<SysDict>) -> impl IntoResponse {
//     let mut rb = &state.batis;
//     let mut data = item;
//     data.create_date = Option::from(DateTime::now());
//     let dict_result = SysDict::insert(&mut rb, &data).await;
//     return Json(dict_result);
// }
//
// pub async fn delete_dict(State(state): State<Arc<AppState>>, Json(item): Json<SysDict>) -> impl IntoResponse {
//     let mut rb = &state.batis;
//     let data = item;
//     let dict_result = SysDict::delete_by_column(&mut rb, "id", data.id).await;
//     return Json(dict_result);
// }


