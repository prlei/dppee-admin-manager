use std::sync::Arc;
use axum::extract::State;
use rbatis::RBatis;
use rbatis::sql::{Page, PageRequest};
use da_manager::APPLICATION_CONTEXT;
use crate::controller::vo::dict_vo::DictPageVO;
use crate::entity::sys_entity::SysDict;
use crate::{AppState};
use crate::entity::dto::dict::SysDictDTO;
use crate::errors::Result;

pub struct SysDictService {}

impl SysDictService {
    pub async fn page(&self, arg: &DictPageVO) -> Result<Page<SysDictDTO>> {
        let sys_dict_service = &APPLICATION_CONTEXT.init_service.sys_dict_service;

        let mut rb = APPLICATION_CONTEXT.rb;

        let data = SysDict::select_page(&mut rb, &PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))).await?;

        let page = Page::<SysDictDTO>::from(data);
        Ok(page)
    }
}
