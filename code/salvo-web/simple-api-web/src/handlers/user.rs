use std::sync::Mutex;

use crate::{models::user::User, prelude::*};

#[handler]
pub async fn get_all_user(depot: &mut Depot) -> Result<String> {
    let current_user = depot.get::<&str>("current_user")
        .copied()
        .map_err(|_| Error::Generic("A error".into()))?;
    let users: Vec<User> = depot.obtain::<Mutex<Vec<User>>>()
        .unwrap()
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|user| user.is_activate())
        .collect();

    Ok(format!("{} User has {} person", current_user, users.len()))
}

#[handler]
pub async fn get_user_by_id(req: &mut Request, depot: &mut Depot) -> Result<String> {
    let id = req.param::<i32>("id")
        .ok_or(Error::Generic("A error".into()))?;
    let user = depot.obtain::<Mutex<Vec<User>>>()
        .unwrap()
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|user| user.is_activate() && user.id == id)
        .ok_or(Error::Generic("User not found".into()));

    match user {
        Ok(user) => Ok(format!("get user id {}", user.id)),
        Err(e) => Err(e)
    }
}

#[handler]
pub async fn delete_user_by_id(req: &mut Request, depot: &mut Depot) -> Result<String> {
    let id = req.param::<i32>("id")
        .ok_or(Error::Generic("A error".into()))?;
    let users = depot.obtain::<Mutex<Vec<User>>>()
        .unwrap()
        .lock()
        .unwrap()
        .clone();

    let user_position = users.iter()
        .position(|u| u.id == id)
        .ok_or(Error::Generic("User not found".into()));

    match user_position {
        Ok(position) => Ok(format!("get user position {}", position)),
        Err(e) => Err(e)
    }
}