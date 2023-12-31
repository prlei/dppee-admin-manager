use std::fmt::Debug;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use serde::Serialize;

pub mod dict_vo;
pub mod sys_vo;

// 统一返回vo
#[derive(Serialize, Debug, Clone)]
pub struct BaseResponse<T>
    where T: Serialize + Debug
{
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

// 处理统一返回
pub fn handle_result(result: Result<ExecResult, Error>) -> BaseResponse<String> {
    match result {
        Ok(_u) => {
            BaseResponse {
                msg: "successful".to_string(),
                code: 0,
                data: Some("None".to_string()),
            }
        }
        Err(err) => {
            BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: Some("None".to_string()),
            }
        }
    }
}