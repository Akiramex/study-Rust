use actix_web::{web, HttpResponse};
use crate::dbaccess::teacher::*;
use crate::error::MyError; 
use crate::state::AppState;
use crate::models::teacher::*;

pub async fn get_all_teachers(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db).await
    .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.0;
    get_teacher_details_db(&app_state.db, teacher_id)
    .await.map(|course| HttpResponse::Ok().json(course))
}

pub async fn post_new_teacher(
    app_state: web::Data<AppState>,
    new_teacher: web::Json<CreateTeacher>
) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, new_teacher.into())
    .await.map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateTeacher>,
    params: web::Path<(i32,)>
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.0;
    update_teacher_details_db(&app_state.db, update_course.into(), teacher_id)
    .await.map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.0;
    delete_teachers_db(&app_state.db, teacher_id)
    .await.map(|course| HttpResponse::Ok().json(course))
}