mod error;
mod prelude;
mod routers;
mod models;
mod handlers;
mod database;
mod utils;

use dotenv::dotenv;
use salvo::prelude::*;
use crate::routers::router;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    dotenv().ok();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    let router = router();

    Server::new(acceptor).serve(router).await;
    
    
}
