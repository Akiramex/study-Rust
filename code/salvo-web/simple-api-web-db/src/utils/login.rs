use serde::Deserialize;
use salvo::session::Session;
use uuid::Uuid;
use crate::prelude::*;
use crate::models::user::User;

const SLAT:&str = "slat";

// 临时写在这里
#[derive(Deserialize)]
pub struct LoginParams {
    pub name: String,
    pub password: String,
}

impl LoginParams {
    pub fn check_params(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(Error::Generic("账号不得为空".to_owned()));
        }
        if self.password.trim().is_empty() {
            return Err(Error::Generic("密码不得为空".to_owned()))
        }
        Ok(())
    }
}

/// 根据输入
/// # 参数
///
/// - `parmars` 用户输入参数
///
/// # 返回值
///
/// 返回一个String类型，表示加密编码后的密码
pub fn password(parmars: &LoginParams) -> String {
    //加盐
    let data = format!("{}{}{}", parmars.name, SLAT, parmars.password);

    //对data进行加密编码: MD5等加密算法
    //todo

    //编成码十六进制数据
    hex::encode(data)
}

pub fn create_user_session(session_id: String, client_ip: String, user: &User) -> Session {

    let new_session_id = if session_id.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        session_id
    };


    Session::new().ins("SessionId", value);
}