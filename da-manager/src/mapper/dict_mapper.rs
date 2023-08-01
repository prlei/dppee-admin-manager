use rbatis::{crud, impl_select_page};
use crate::entity::dict_entity::SysDict;

crud!(SysDict {});
impl_select_page!(SysDict{select_page() =>"
     if !sql.contains('count'):
       order by create_date desc"});

// impl_select_page!(SysDict {select_page(dto: &crate::controller::vo::dict_vo::DictPageVO) =>
//     "`where id!=''`
//       if dto.code!=null:
//          ` and code = #{dto.code}`
//       if dto.name!=null:
//          ` and name = #{dto.name}`
//       if !sql.contains('count'):
//          ` order by create_date `"});