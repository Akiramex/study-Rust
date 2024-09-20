use std::sync::{Arc, Mutex};
use crate::prelude::*;
use crate::models::user::{User, CreateUser};

#[handler]
pub async fn get_all_user(depot: &mut Depot) -> Result<Json<Vec<User>>> {
    let users: Vec<User> = depot.obtain::<Arc<Mutex<Vec<User>>>>()
        .unwrap()
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|u| u.is_activate())
        .collect();
    
    Ok(Json(users))
}

#[handler]
pub async fn get_user_by_id(req: &mut Request, depot: &mut Depot) -> Result<Json<User>> {
    let id = req.param::<i32>("id")
        .ok_or(Error::Generic("A error".into()))?;
    let user = depot.obtain::<Arc<Mutex<Vec<User>>>>()
        .unwrap()
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|u| u.is_activate() && u.id == id)
        .ok_or(Error::Generic("User not found".into()))?;

    Ok(Json(user))
}

#[handler]
pub async fn delete_user_by_id(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<()> {
    let id = req.param::<i32>("id")
        .ok_or(Error::Generic("Error input".into()))?;

    let mut users = depot.obtain_mut::<Arc<Mutex<Vec<User>>>>()
        .unwrap()
        .lock()
        .unwrap();
    // 这里不能clone，否则数据不能更新
    let user_position = users.iter()
        .position(|u| u.id == id)
        .ok_or(Error::Generic("User not found".into()));

    match user_position {
        Ok(position) => {
            users[position].set_delete();

            res.status_code(StatusCode::OK).render("Success delete".to_owned());
            Ok(())
        },
        Err(e) => {
            res.status_code(StatusCode::NOT_FOUND).render(e.to_string());
            Ok(())
        }
    }
}

use chrono::Utc;
#[handler]
pub async fn create_user(req: &mut Request, depot: &mut Depot) -> Result<String> {
    let new_user = req.parse_json::<CreateUser>()
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    let mut users = depot.obtain_mut::<Arc<Mutex<Vec<User>>>>()
        .unwrap()
        .lock()
        .unwrap();
        // 这里不能clone，否则数据不能更新
    let new_user = User {
        id: users.len() as i32 + 1, 
        name: new_user.name.clone(), 
        password: new_user.password.clone(), 
        state: 0,
        time: Utc::now().naive_utc(),
    };
    users.push(new_user);

    Ok("Success Add".to_owned())
}
