use salvo::prelude::*;
use crate::prelude::*;
use crate::models::user::*;
use crate::models::ResponseInfo;
use crate::database::user::*;
use salvo::http::ParseError;

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) -> Result<()> {
    let id = req.try_query::<i32>("id");
    match id {
        Err(err) => match err {
            ParseError::NotExist => {
                let users = get_all_user_db().await?;

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
    let new_user = req.parse_form::<CreateUser>().await?;

    create_user_db(new_user).await?;

    let resp = ResponseInfo {
        msg: "Success Create".to_owned(),
        ..Default::default()
    };
    res.render(Json(resp));
    Ok(())
}