pub mod controller;
pub mod entity;
pub mod mapper;
pub mod service;
pub mod errors;

use std::sync::Arc;
use axum::{routing::post, Router};
use axum::routing::get;
use once_cell::sync::{Lazy, OnceCell};
use rbatis::RBatis;
use tokio::fs::read_to_string;
use state::Container;
use da_manager::init_context;
use crate::entity::config::ApplicationConfig;
use crate::service::ServiceContext;
use crate::service::sys_dist_service::SysDictService;


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

    init_context().await;

    // let rb = init_db().await;
    // let shared_state = Arc::new(AppState { /* ... */ batis: rb.clone() });

    let app = Router::new()
        .nest("/api", Router::new()
            .route("/user_list", post(controller::sys_contorller::user_list))

            .route("/query_dict_page", post(controller::sys_contorller::query_dict_page)));

            // .route("/query_user_by_id/:id", get(controller::sys_contorller::query_user_by_id))
            // .route("/user_save", post(controller::sys_contorller::user_save))
            // .route("/add_dict", post(controller::sys_contorller::add_dict))
            // .route("/update_dict", post(controller::sys_contorller::update_dict))
            // .route("/delete_dict", post(controller::sys_contorller::delete_dict))
            // .route("/query_dict_page", post(controller::sys_contorller::query_dict_page))
            // .route("/query_dict", post(controller::sys_contorller::query_dict))
            // .route("/query_dict_by_id/:id", get(controller::sys_contorller::query_dict_by_id))
            // .with_state(shared_state));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
