mod error;
mod prelude;
mod routers;
mod models;
mod handlers;
mod database;
mod utils;
mod middleware;

use std::sync::OnceLock;
use sqlx::{PgPool, Pool};
use dotenv::dotenv;
use salvo::prelude::*;
use std::env;

use crate::routers::router;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub fn db_pool() -> &'static PgPool {
    DB_POOL.get().unwrap()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    dotenv().ok();

    // 同步加载失败了QAQ，只能切回来用异步加载了
    let database_url = env::var("DATABASE_URL").expect("DATABASE is no found");
    let pool = Pool::connect(&database_url).await.unwrap();
    DB_POOL.set(pool).unwrap();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    let router = router();

    Server::new(acceptor).serve(router).await;
    
}
