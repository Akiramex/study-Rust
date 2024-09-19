mod error;
mod prelude;
mod routes;
mod handlers;
mod models;

use std::sync::Mutex;

use crate::prelude::*;
use crate::routes::route;
use crate::models::user::User;

#[handler]
async fn set_user(depot: &mut Depot) {
    depot.insert("current_user", "Elon Musk").inject(Mutex::new(vec![User::default(), User::default(), User::default()]));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    let route = Router::new().hoop(set_user).push(route());

    Server::new(acceptor).serve(route).await;
}
