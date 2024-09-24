use salvo::prelude::*;
use crate::prelude::*;
use crate::models::user::*;
use crate::database::user::*;

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) -> Result<()> {
    let id: Option<String> = req.query("id");
    match id {
        None => {
            let users = get_all_user_db().await?;

            let res_users: Vec<ResponseUser> = users.into_iter()
                .map(ResponseUser::from)
                .collect();
            res.render(Json(res_users));
            Ok(())
        },
        Some(id) => match id.parse::<i32>() {
            Err(e) => {
                Err(Error::Generic(e.to_string()))
            }
            Ok(id) => {
                let user: ResponseUser = get_user_by_id_db(id).await?.into();

                res.render(Json(user));
                Ok(())
            },

        }
    }
}

#[handler]
pub async fn delete_user_by_id(req: &mut Request) -> Result<String> {
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

            Ok("Success delete".into())
        }
    }
}

#[handler]
pub async fn update_user_by_id(req: &mut Request) -> Result<String> {
    let id: Option<i32> = req.query("id");

    let update_user: UpdateUser = req.parse_json::<UpdateUser>().await?;

    match id {
        None => {
            Err(Error::Generic("Please input the correct query".to_owned()))
        },
        Some(id) => {
            update_user_by_id_db(id, update_user).await?;

            Ok("Success update".into())
        }
    }
}

#[handler]
pub async fn create_user(req: &mut Request) -> Result<String> {
    let new_user = req.parse_json::<CreateUser>().await?;

    create_user_db(new_user).await?;

    Ok("Success create".into())
}