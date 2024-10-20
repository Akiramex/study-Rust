use uuid::Uuid;

use crate::models::user::*;
use crate::database::redis::*;
use crate::prelude::*;

/// 对密码进行加密操作
/// # 参数
///
/// - `parmars` 用户输入参数
///
/// # 返回值
///
/// 返回一个String类型，表示加密编码后的密码
const SLAT: &str = "slat";
pub fn password(parmars: &LoginParams) -> String {
    //加盐
    let data = format!("{}{}{}", parmars.name, SLAT, parmars.password);

    //对data进行加密编码: MD5等加密算法
    //todo

    //编成码十六进制数据
    hex::encode(data)
}

pub fn create_user_session(session_id: String, _client_ip: String, _user: &User) -> Result<String> {

    let session_id = if session_id.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        session_id
    };

    // todo 把client_ip 和 user 弄成一个结构体序列化放到 value 里面 
    rs_set_value(&session_id, "")
        .map_err(|err| Error::Generic(err.to_string()))?;

    Ok(session_id)
}

pub fn compare_session(user_session_id: &str) -> bool {
    if let Ok(info) = rs_get_value(user_session_id) {
        // 刷新一下
        let _ = rs_set_value(user_session_id, &info);
        true
    } else {
        false
    }
}