mod error;
mod prelude;
//mod routers;
mod models;
mod handlers;
mod database;

use dotenv::dotenv;
use std::env;
use std::sync::OnceLock;
use sqlx::{PgPool, Pool};
use salvo::prelude::*;

pub static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub fn dp_pool() -> &'static PgPool {
    DB_POOL.get().unwrap()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE is no found");
    
    let pool = Pool::connect(&database_url).await.unwrap();
    
    DB_POOL.set(pool).unwrap();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    let router = Router::new();

    Server::new(acceptor).serve(router).await;
}
