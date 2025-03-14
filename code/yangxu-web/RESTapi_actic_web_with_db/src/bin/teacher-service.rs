use actix_web::{web, App, HttpServer};
use std::{env, io};
use std::sync::Mutex;

#[path ="../handlers.rs"]
mod handlers;
#[path ="../routers.rs"]
mod routers;
#[path ="../state.rs"]
mod state;
#[path ="../models.rs"]
mod models;
#[path ="../db_access.rs"]
mod db_access;
#[path ="../error.rs"]
mod error;

use routers::*;
use state::AppState;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> io::Result<()>{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE is no found");

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".into(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
