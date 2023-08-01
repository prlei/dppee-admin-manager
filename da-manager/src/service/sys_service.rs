use log::{info, log};

use crate::controller::vo::sys_vo::UserLoginVO;
use crate::db_pool;
use crate::entity::sys_entity::SysUser;
use crate::errors::{Result, Error};

pub struct SysService {}

impl SysService {
    pub async fn query_user_by_mobile(&self, arg: &UserLoginVO) -> Result<SysUser> {
        info!("----  {:?}", &arg.mobile);
        let user_result = SysUser::select_by_column(db_pool!(), "mobile", &arg.mobile).await?
            .into_iter()
            .next();
        let u_data = SysUser::select_by_column(db_pool!(), "mobile", &arg.mobile).await;
        // 判断用户是否存在
        let user_result = user_result.ok_or_else(|| Error::from(format!("账号:{} 不存在!", arg.mobile)))?;
        // 判断用户是否被禁用
        if user_result.status.eq(&Some(0)) {
            return Err(Error::from("账户被禁用！"))
        }

        match u_data {
            Ok(d) => {
                if d.len() == 0 {
                    return Err(Error::from("用户不存在"))
                }
                let user = d.get(0).unwrap().clone();
                let id = user.id.unwrap().to_string();
                let username = user.real_name.unwrap();
                let password = user.password.unwrap();

                if password.ne(&arg.password) {
                    return Err(Error::from("密码错误"));
                }
            }
            Err(err) => {
                return Err(Error::from("查询用户异常"));
            }
        };


        // 判断密码是否正确

        // 生成token

        // 生成userinfo信息并返回




        // 其他 判断登录频繁、判断验证码

        return Ok(user_result);
    }
}

