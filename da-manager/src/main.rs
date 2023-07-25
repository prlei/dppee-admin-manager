pub mod controller;
pub mod entity;
pub mod mapper;

use std::sync::Arc;
use axum::{routing::post, Router};
use axum::routing::get;
use rbatis::RBatis;

pub struct AppState {
    pub batis: RBatis,
}

pub async fn init_db() -> RBatis {
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:rootroot@127.0.0.1:3306/dppee").unwrap();
    return rb;
}

#[tokio::main]
async fn main() {

    let rb = init_db().await;
    let shared_state = Arc::new(AppState { /* ... */ batis: rb.clone() });

    let app = Router::new()
        .nest("/api", Router::new()
            .route("/user_list", post(controller::sys_contorller::user_list))
            .route("/query_user_by_id/:id", get(controller::sys_contorller::query_user_by_id))
            .route("/user_save", post(controller::sys_contorller::user_save))
            .route("/add_dict", post(controller::sys_contorller::add_dict))
            .route("/update_dict", post(controller::sys_contorller::update_dict))
            .route("/delete_dict", post(controller::sys_contorller::delete_dict))
            .route("/query_dict_page", post(controller::sys_contorller::query_dict_page))
            .route("/query_dict", post(controller::sys_contorller::query_dict))
            .route("/query_dict_by_id/:id", get(controller::sys_contorller::query_dict_by_id))
            .with_state(shared_state));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
