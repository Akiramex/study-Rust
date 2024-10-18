use salvo::prelude::*;
use crate::prelude::*;
use crate::models::user::*;
use crate::models::ResponseInfo;
use crate::database::{
    user::*,
    redis::*,
};
use crate::utils::login::password;
use salvo::http::ParseError;

const KEY_USER: &str = "user";

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) -> Result<()> {
    let id = req.try_query::<i32>("id");
    match id {
        Err(err) => match err {
            ParseError::NotExist => {
                
                let users: Vec<User> = match rs_get_value(KEY_USER) {
                    Err(_e) => {
                        // 不知道怎么判断缓存未击中还是redis错误了s

                        // 缓存未击中，查询数据库，并添加缓存
                        info!("Cache not attacked!");
                        let users = get_all_user_db().await?;

                        let string_user = serde_json::to_string(&users).unwrap();
                        let _ = rs_set_value(KEY_USER, &string_user);
                        users
                        
                    }
                    Ok(users) => {
                        // 缓存击中，从缓存中取出，反序列化
                        info!("Cache attacked!");
                        serde_json::from_str(&users).unwrap()
                    }
                };

                let res_users: Vec<ResponseUser> = users.into_iter()
                    .map(ResponseUser::from)
                    .collect();

                let resp = ResponseInfo {
                    code: 0,
                    msg: String::new(),
                    total: 0,
                    data: res_users
                };

                res.render(Json(resp));
                Ok(())
            }
            _ => {
                Err(Error::SalvoPE(err))
            }
        }
        Ok(id) => {
            let user: ResponseUser = get_user_by_id_db(id).await?.into();
                
            let resp = ResponseInfo {
                code: 0,
                msg: String::new(),
                total: 0,
                data: user
            };

            res.render(Json(resp));
            Ok(())
        }
    }
}

#[handler]
pub async fn delete_user_by_id(req: &mut Request, res: &mut Response) -> Result<()> {
    let id: Option<i32> = req.query("id");

    let temp = UpdateUser {
        status:Some(1),
        ..Default::default()
    };

    match id {
        None => {
            Err(Error::Generic("Please input the correct query".to_owned()))
        },
        Some(id) => {
            update_user_by_id_db(id, temp).await?;

            let resp = ResponseInfo {
                msg: "Success Delete".to_owned(),
                ..Default::default()
            };
            res.render(Json(resp));
            Ok(())
        }
    }
}

#[handler]
pub async fn update_user_by_id(req: &mut Request, res: &mut Response) -> Result<()> {
    let id: Option<i32> = req.query("id");

    let update_user: UpdateUser = req.parse_json::<UpdateUser>().await?;

    match id {
        None => {
            Err(Error::Generic("Please input the correct query".to_owned()))
        },
        Some(id) => {
            update_user_by_id_db(id, update_user).await?;

            let resp = ResponseInfo {
                msg: "Success Update".to_owned(),
                ..Default::default()
            };
            res.render(Json(resp));
            Ok(())
        }
    }
}

#[handler]
pub async fn create_user(req: &mut Request, res: &mut Response) -> Result<()> {
    let mut new_user = req.parse_form::<CreateUser>().await?;

    let parmas = LoginParams {
        name: new_user.name.clone(),
        password: new_user.password.clone(),
    };

    // 密码加密
    let hash_password = password(&parmas);

    new_user.password = hash_password;

    create_user_db(new_user).await?;

    // 删除缓存
    let _ = rs_delete_key(KEY_USER);

    let resp = ResponseInfo {
        msg: "Success Create".to_owned(),
        ..Default::default()
    };

    res.render(Json(resp));
    Ok(())
}