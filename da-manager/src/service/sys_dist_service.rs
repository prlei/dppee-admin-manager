use std::sync::Arc;

use rbatis::sql::{Page, PageRequest};
use crate::{AppState, CONTEXT, pool};

use crate::controller::vo::dict_vo::DictPageVO;
use crate::entity::dto::dict::SysDictDTO;
use crate::entity::sys_entity::SysDict;
use crate::errors::Result;

pub struct SysDictService {}

impl SysDictService {
    pub async fn page(&self, arg: &DictPageVO) -> Result<Page<SysDictDTO>> {
        println!("{:?}", pool!());
        let state = CONTEXT.get::<Arc<AppState>>();
        let mut rb = &state.batis;
        let data = SysDict::select_page(pool!(), &PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))).await?;
        let page = Page::<SysDictDTO>::from(data);
        Ok(page)
    }
    pub async fn fund_by_id(&self, arg: &String) -> Result<Option<SysDict>> {
        let state = CONTEXT.get::<Arc<AppState>>();
        let mut rb = &state.batis;
        Ok(SysDict::select_by_column(&mut rb, "id", arg)
            .await?
            .into_iter()
            .next())
    }
    pub async fn find_dict(&self, arg: &DictPageVO) -> Result<Option<SysDict>> {
        let state = CONTEXT.get::<Arc<AppState>>();
        let mut rb = &state.batis;
        let data = SysDict::select_by_column(&mut rb, "code", &arg.code).await?
            .into_iter()
            .next();
        Ok(data)
    }

    pub async fn update_dict(&self, arg: &SysDict) -> Result<u64> {
        let state = CONTEXT.get::<Arc<AppState>>();
        let mut rb = &state.batis;
        let data = SysDict::update_by_column(&mut rb, &arg,"id").await?.rows_affected;
        Ok(data)
    }

    pub async fn create_dict(&self, arg: &SysDict) -> Result<u64> {
        let state = CONTEXT.get::<Arc<AppState>>();
        let mut rb = &state.batis;
        let data = SysDict::insert(&mut rb, &arg).await?.rows_affected;
        Ok(data)
    }

    pub async fn del_dict(&self, arg: &SysDict) -> Result<u64> {
        let state = CONTEXT.get::<Arc<AppState>>();
        let mut rb= &state.batis;
        let data  = SysDict::delete_by_column(&mut rb, "id", &arg.id).await?;
        Ok(data.rows_affected)
    }
}
