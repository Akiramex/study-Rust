use error::AppError;
use salvo::prelude::*;

mod error;
mod prelude;
use crate::prelude::*;


#[handler]
async fn get_all_user() -> &'static str{
    "abc"
}

#[handler]
async fn get_user_by_id(req: &mut Request) -> Result<String> {
    let id = req.param::<i64>("id")
        .ok_or(AppError::Generic("a".into()))?;

    Ok(format!("get user {id}"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .push(
            Router::with_path("training/mobile/api")
                .push(
                    Router::with_path("user")
                        .get(get_all_user)
                        .push(Router::with_path("<id>").get(get_user_by_id))
                )
        );

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
