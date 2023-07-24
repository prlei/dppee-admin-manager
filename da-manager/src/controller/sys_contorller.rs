use std::string::String;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use once_cell::sync::Lazy;
use rbatis::{crud, impl_select, RBatis};
use rbatis::rbdc::datetime::DateTime;
use rbdc_mysql::driver::MysqlDriver;
use serde_json::json;

use crate::AppState;
use crate::entity::sys_entity::{SysUser, SysUserQuery};

pub static RB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

crud!(SysUser{});
impl_select!(SysUser{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});

// curl localhost:8080/api/user_save -X POST -d '{"mobile": "bb", "password": "apple"}' --header "Content-Type: application/json"
pub async fn user_save(Json(payload): Json<SysUser>) -> impl IntoResponse {
    RB.link(MysqlDriver{},"mysql://root:123456@localhost:3306/dppee").await.unwrap();

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

    // let data = SysUser::select_by_id(&mut RB, "1".to_string()).await;
    // println!("select_by_id = {}", json!(data));

    return Json(json!({"mobile": user.mobile, "password": user.password}));
}

// pub async fn user_save(Json(payload): Json<SysUser>) -> impl IntoResponse {
//     let user = payload;
//     println!("{:?},{:?}", user.password, user.mobile);
//     return Json(json!({"mobile": user.mobile, "password": user.password}));
// }

// curl localhost:8080/api/query_user_by_id/1
pub async fn query_user_by_id(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    let mut rb = &state.batis;
    let user_result = SysUser::select_by_id(&mut rb, id).await;
    // println!(">>>>> user_result = {:?}", user_result);
    return Json(user_result);
}

// curl localhost:8080/api/user_list -X POST -d '{"id":1}' --header "Content-Type: application/json"
pub async fn user_list(State(state): State<Arc<AppState>>, Json(arg): Json<SysUserQuery>) -> impl IntoResponse {

    let user_arg = arg;

    let mut rb = &state.batis;

    let user_result = SysUser::select_by_id(&mut rb, user_arg.id.to_string()).await;

    let count: u64 = rb
        .query_decode("select count(1) as count from sys_user", vec![])
        .await
        .unwrap();

    println!(">>>>> count = {}", count);

    println!(">>>>> user_result = {:?}", user_result);

    return Json(user_result);

    // match user_result {
    //     Ok(user) => {
    //         let mut sys_user = user.unwrap();
    //         sys_user.password = Some("aaaa".to_string());
    //         let result = SysUser::update_by_column(&mut rb, &sys_user, "id").await;
    //
    //         Json(handle_result(result))
    //     }
    //     Err(err) => {
    //         let resp = BaseResponse {
    //             msg: err.to_string(),
    //             code: 1,
    //             data: None,
    //         };
    //         Json(resp)
    //     }
    // }


    // let rb = RBatis::new();
    // rb.link(MysqlDriver {}, "mysql://root:123456@localhost:3306/dppee").await.unwrap();
    //
    // let table: Option<SysUser> = rb
    //     .query_decode("select * from sys_user limit ?", vec![to_value!(1)])
    //     .await
    //     .unwrap();
    // let count: u64 = rb
    //     .query_decode("select count(1) as count from biz_activity", vec![])
    //     .await
    //     .unwrap();
    // sleep(Duration::from_secs(1)).await;
    // println!(">>>>> table={:?}", table);
    // println!(">>>>> count={}", count);
}

