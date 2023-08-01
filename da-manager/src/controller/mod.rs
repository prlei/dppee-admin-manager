use axum::Router;
use axum::routing::{get, post};

pub mod dict_controller;
pub mod vo;
pub mod sys_controller;

pub fn api_router() -> Router {
    sys_router().merge(dict_router())
}

pub fn dict_router() -> Router {
    Router::new()
        .route("/test", post(dict_controller::test))
        .route("/query_dict_page", post(dict_controller::query_dict_page))
        .route("/query_dict_by_id/:id", get(dict_controller::query_dict_by_id))
        .route("/query_dict", post(dict_controller::query_dict))
        .route("/edit_dict", post(dict_controller::edit_dict))
        .route("/add_dict", post(dict_controller::add_dict))
        .route("/delete_dict", post(dict_controller::delete_dict))
}

pub fn sys_router() -> Router {
    Router::new()
        .route("/login", post(sys_controller::login))
}