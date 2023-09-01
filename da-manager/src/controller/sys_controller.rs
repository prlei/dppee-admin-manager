use axum::Json;
use axum::response::IntoResponse;
use log::info;

use crate::CONTEXT;
use crate::controller::vo::sys_vo::UserLoginVO;
use crate::service::ServiceContext;
use crate::utils::result_body::ResultBody;

// sys start
pub async fn login(Json(item): Json<UserLoginVO>) -> impl IntoResponse {
    info!("user query params: {:?}", &item.mobile);
    let service_context = CONTEXT.get::<ServiceContext>();
    let result = service_context.sys_service.query_user_by_mobile(&item).await;
    ResultBody::from_result(&result).resp_json()
}

// sys end