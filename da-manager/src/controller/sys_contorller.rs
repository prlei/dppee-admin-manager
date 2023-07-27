use std::string::String;

use axum::{Json, Router};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use rbatis::rbdc::datetime::DateTime;

use crate::CONTEXT;
use crate::controller::vo::dict_vo::DictPageVO;
use crate::entity::sys_entity::{DictEditDTO, SysDict};
use crate::service::ServiceContext;

// dict start

//curl localhost:8080/api/query_dict_page -X POST -d '{"page_no":1, "page_size":3}' --header "Content-Type: application/json"
pub async fn query_dict_page(Json(item): Json<DictPageVO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let sys_dict = service_context.sys_dict_service.page(&item).await;
    return Json(sys_dict);
}

// curl localhost:8080/api/query_dict_by_id/1
pub async fn query_dict_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let sys_dict = service_context.sys_dict_service.fund_by_id(&id).await;
    return Json(sys_dict);
}

// curl localhost:8080/api/query_dict -X POST --header "Content-Type: application/json" -d '{"code":"type"}'
pub async fn query_dict(Json(item): Json<DictPageVO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let sys_dict = service_context.sys_dict_service.find_dict(&item).await;
    return Json(sys_dict);
}

// curl localhost:8080/api/edit_dict -X POST -d '{"id":"3", "code":"pici5"}' --header "Content-Type: application/json"
pub async fn edit_dict(item: Json<DictEditDTO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let data = SysDict::from(&item.0);
    let sys_dict = service_context.sys_dict_service.update_dict(&data).await;
    return Json(sys_dict);
}

pub async fn add_dict(item: Json<DictEditDTO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let mut dict = SysDict::from(&item.0);
    dict.create_date =  Option::from(DateTime::now());
    let dict_result = service_context.sys_dict_service.create_dict(&dict).await;
    return Json(dict_result);
}

pub async fn delete_dict(item: Json<DictEditDTO>) -> impl IntoResponse {
    let service_context = CONTEXT.get::<ServiceContext>();
    let dict = SysDict::from(&item.0);
    let dict_result = service_context.sys_dict_service.del_dict(&dict).await;
    return Json(dict_result);
}

// dict end



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


// // curl localhost:8080/api/user_save -X POST -d '{"mobile": "bb", "password": "apple"}' --header "Content-Type: application/json"
// pub async fn user_list(Json(payload): Json<SysUser>) -> impl IntoResponse {
//
//     // 使用相对路径读取 app.yaml 内容为字符串
//     let content = read_to_string("application.yml").await.unwrap();
//     // serde_yaml 解析字符串为 User 对象
//     let config: ApplicationConfig = ApplicationConfig::new(content.as_str());
//
//     log::info!("Sysuser : {:?}", payload);
//     let user = SysUser{
//         id: Option::from(12),
//         gmt_create: Option::from(DateTime::now()),
//         gmt_modified: Option::from(DateTime::now()),
//         username: Option::from(String::from("john")),
//         gender: Option::from(1),
//         email: Option::from(String::from("john@example.com")),
//         status_id:Option::from(16),
//         sort: Option::from(1),
//         user_no: Option::from(17),
//         mobile: Option::from(String::from("lisa")),
//         real_name: Option::from(String::from("ad")),
//         remark: Option::from(String::from("ae")),
//         password: Option::from(String::from("af")),
//     };
//
//     return Json(json!({"mobile": user.mobile, "password": user.password}));
// }

pub fn init_router() -> Router {
    Router::new()
        // .route("/user_list", post(user_list))
        .route("/query_dict_page", post(query_dict_page))
        .route("/query_dict_by_id/:id", get(query_dict_by_id))
        .route("/query_dict", post(query_dict))
        .route("/edit_dict", post(edit_dict))
        .route("/add_dict", post(add_dict))
        .route("/delete_dict", post(delete_dict))
}
