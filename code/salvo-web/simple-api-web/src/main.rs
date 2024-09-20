mod error;
mod prelude;
mod routes;
mod handlers;
mod models;

use std::sync::{Arc, Mutex};

use crate::prelude::*;
use crate::routes::route;
use crate::models::user::User;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    let users = Arc::new(Mutex::new(vec![
        User {
            id: 1,
            ..Default::default()
        },
        User {
            id: 2,
            ..Default::default()
        },
        User {
            id: 3,
            ..Default::default()
        }]));

    let router = Router::new()
        .hoop(affix_state::inject(users).insert("current_user", "Elon Musk"))
        .push(route());

    Server::new(acceptor).serve(router).await;
}
