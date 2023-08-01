use rbatis::sql::{Page, PageRequest};

use crate::db_pool;
use crate::controller::vo::dict_vo::DictPageVO;
use crate::entity::dto::dict::SysDictDTO;
use crate::entity::dict_entity::SysDict;
use crate::errors::AppResult;

pub struct SysDictService {}

impl SysDictService {
    pub async fn page(&self, arg: &DictPageVO) -> AppResult<Page<SysDictDTO>> {
        let data = SysDict::select_page(db_pool!(), &PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))).await?;
        let page = Page::<SysDictDTO>::from(data);
        Ok(page)
    }
    pub async fn fund_by_id(&self, arg: &String) -> AppResult<Option<SysDict>> {
        Ok(SysDict::select_by_column(db_pool!(), "id", arg)
            .await?
            .into_iter()
            .next())
    }
    pub async fn find_dict(&self, arg: &DictPageVO) -> AppResult<Option<SysDict>> {
        let data = SysDict::select_by_column(db_pool!(), "code", &arg.code).await?
            .into_iter()
            .next();
        Ok(data)
    }

    pub async fn update_dict(&self, arg: &SysDict) -> AppResult<u64> {
        let data = SysDict::update_by_column(db_pool!(), &arg,"id").await?.rows_affected;
        Ok(data)
    }

    pub async fn create_dict(&self, arg: &SysDict) -> AppResult<u64> {
        let data = SysDict::insert(db_pool!(), &arg).await?.rows_affected;
        Ok(data)
    }

    pub async fn del_dict(&self, arg: &SysDict) -> AppResult<u64> {
        let data  = SysDict::delete_by_column(db_pool!(), "id", &arg.id).await?;
        Ok(data.rows_affected)
    }
}
