use std::string::String;

use axum::Json;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use log::info;
use rbatis::rbdc::datetime::DateTime;

use crate::CONTEXT;
use crate::controller::vo::dict_vo::DictPageVO;
use crate::entity::dict_entity::{DictEditDTO, SysDict};
use crate::service::ServiceContext;
use crate::utils::result_body::ResultBody;

// dict start

//curl localhost:8080/api/query_dict_page -X POST -d '{"page_no":1, "page_size":3}' --header "Content-Type: application/json"
pub async fn query_dict_page(Json(item): Json<DictPageVO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let sys_dict = service_context.sys_dict_service.page(&item).await;
    // return Json(sys_dict);
    ResultBody::from_result(&sys_dict).resp_json()
}

// curl localhost:8080/api/query_dict_by_id/1
pub async fn query_dict_by_id(headers: HeaderMap, Path(id): Path<String>) -> impl IntoResponse {
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    log::info!("token ------  : {}", token);
    let service_context = CONTEXT.get::<ServiceContext>();
    let sys_dict = service_context.sys_dict_service.fund_by_id(&id).await;
    ResultBody::from_result(&sys_dict).resp_json()
}

// curl localhost:8080/api/query_dict -X POST --header "Content-Type: application/json" -d '{"code":"type"}'
pub async fn query_dict(Json(item): Json<DictPageVO>) -> impl IntoResponse {
    info!("user query params: {:?}", &item.code);
    let service_context = CONTEXT.get::<ServiceContext>();
    let sys_dict = service_context.sys_dict_service.find_dict(&item).await;
    ResultBody::from_result(&sys_dict).resp_json()
}

// curl localhost:8080/api/edit_dict -X POST -d '{"id":"3", "code":"pici5"}' --header "Content-Type: application/json"
pub async fn edit_dict(item: Json<DictEditDTO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let data = SysDict::from(&item.0);
    let sys_dict = service_context.sys_dict_service.update_dict(&data).await;
    ResultBody::from_result(&sys_dict).resp_json()
}

pub async fn add_dict(item: Json<DictEditDTO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let mut dict = SysDict::from(&item.0);
    dict.create_date =  Option::from(DateTime::now());
    let dict_result = service_context.sys_dict_service.create_dict(&dict).await;
    ResultBody::from_result(&dict_result).resp_json()
}

pub async fn delete_dict(item: Json<DictEditDTO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let dict = SysDict::from(&item.0);
    let dict_result = service_context.sys_dict_service.del_dict(&dict).await;
    ResultBody::from_result(&dict_result).resp_json()
}

// dict end

pub async fn test(Json(item): Json<DictPageVO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let sys_dict = service_context.sys_dict_service.find_dict(&item).await;
    ResultBody::from_result(&sys_dict).resp_json()
}
