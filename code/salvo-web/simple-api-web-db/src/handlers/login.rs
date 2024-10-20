use salvo::prelude::*;

use crate::prelude::*;
use crate::database::{
    user::get_user_by_name,
    redis::rs_set_value,
};

use crate::utils::login::*;
use crate::models::{
    user::LoginParams,
    ResponseInfo,
};

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
    let param = req.parse_form::<LoginParams>().await?;

    // 检查查询参数
    param.check_params()?;

    let user = get_user_by_name(&param.name).await?;
    let ret = password(&param);
    if ret != user.password {
        return Err(Error::Generic("密码错误".to_owned()));
    }
    let session = create_user_session(String::new(), String::new(), &user)?;

    
    res.render(Json(ResponseInfo::<String> {
        code : 0,
        total: 0,
        msg: "Success login".to_owned(),
        data: session,
    }));

    Ok(())
}