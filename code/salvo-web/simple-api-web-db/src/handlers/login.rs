use salvo::prelude::*;

use crate::prelude::*;
use crate::database::user::get_user_by_name;
use crate::utils::login::*;


#[handler]
pub async fn login(req: &mut Request) -> Result<String> {
    let param = req.parse_json::<LoginParams>().await?;

    // 检查查询参数
    param.check_params()?;

    let user = get_user_by_name(&param.name).await?;
    let ret = password(&param);
    if ret != user.password {
        return Err(Error::Generic("密码错误".to_owned()));
    }
    
    Ok("登录成功".to_owned())
}