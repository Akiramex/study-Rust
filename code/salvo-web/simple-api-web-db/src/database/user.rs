use crate::db_pool;
use crate::prelude::*;
use crate::models::user::*;

pub async fn get_all_user_db() -> Result<Vec<User>>{
    let rows: Vec<User> = sqlx::query_as!(
        User,
        "SELECT * FROM sys_user WHERE status = 0",
    )
    .fetch_all(db_pool())
    .await?;

    Ok(rows)
}

pub async fn get_user_by_name(user_name: &String) -> Result<User> {
    let row = sqlx::query_as!(
        User,
        "SELECT * FROM sys_user WHERE name = $1 and status = 0",
        user_name
    )
    .fetch_one(db_pool())
    .await?;

    Ok(row)
}

pub async fn get_user_by_id_db(user_id: i32) -> Result<User> {
    let row = sqlx::query_as!(
        User,
        "SELECT * FROM sys_user WHERE id = $1 and status = 0",
        user_id
    )
    .fetch_one(db_pool())
    .await?;

    Ok(row)
}

#[allow(dead_code)]
pub async fn delete_user_by_id_db(user_id: i32) -> Result<User> {
    let row = sqlx::query_as!(
        User,
        "SELECT * FROM sys_user WHERE id = $1",
        user_id
    )
    .fetch_one(db_pool())
    .await?;

    Ok(row)
}

pub async fn update_user_by_id_db(user_id: i32, update_user: UpdateUser) -> Result<User> {
    let current_user_row = sqlx::query_as!(
        User,
        "SELECT * FROM sys_user WHERE id = $1 and status = 0",
        user_id
    )
    .fetch_one(db_pool())
    .await?;

    let name = if let Some(name) = update_user.name { name } else { current_user_row.name };

    let password = if let Some(password) = update_user.password { password } else { current_user_row.password };
    
    let status = if let Some(status) = update_user.status { status } else { current_user_row.status };

    let user_row = sqlx::query_as!(
        User,
        "UPDATE sys_user SET name = $1, password = $2, status = $3 WHERE id = $4
        RETURNING id, name, password, status, time",
        name,
        password,
        status,
        user_id,
    )
    .fetch_one(db_pool())
    .await?;

    Ok(user_row)
}

pub async fn create_user_db(new_user: CreateUser) -> Result<User> {
    let row = sqlx::query_as!(
        User,
        "INSERT INTO sys_user (name, password) 
        VALUES ($1, $2)
        RETURNING id, name, password, status, time",
        new_user.name,
        new_user.password,
    )
    .fetch_one(db_pool())
    .await?;

    Ok(row)
}