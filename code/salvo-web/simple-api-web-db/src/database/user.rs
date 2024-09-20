use crate::dp_pool;
use crate::prelude::*;
use crate::models::user::*;


pub async fn get_all_user_db() {

}

pub async fn get_user_by_id_db() {
    
}

pub async fn delete_user_by_id_db() {
    
}

pub async fn create_user_db(new_user: CreateUser) -> Result<User>{
    let row = sqlx::query!(
        "INSERT INTO sys_user (name, password) 
        VALUES ($1, $2)
        RETURNING id, name, password, status, time",
        new_user.name,
        new_user.password,
    )
    .fetch_one(dp_pool())
    .await?;

    Ok(User{
        id: row.id,
        name: row.name.clone(),
        password: row.password.clone(),
        status: row.status,
        time: row.time,
    })
}