use crate::models::{TeacherRegisterForm, TeacherReponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;
use crate::error;
pub async fn get_all_teachers(tmpl: web::Data<tera::Tera>) 
-> Result<HttpResponse, Error>{
    let awc_client = awc::Client::default();

    let res = awc_client
        .get("HTTP://localhost:3000/teachers/")
        .send()
        .await
        .unwrap()
        .json::<Vec<TeacherReponse>>()
        .await
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("teachers", &res);

    let s = tmpl
        .render("teachers.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error.".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_register_teachers() {

}

pub async fn handle_register() {

}