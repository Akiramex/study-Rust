use salvo::{catcher::Catcher, prelude::*};

#[handler]
async fn hello() -> &'static str {
    "Hello world"
}

#[handler]
async fn error500(res: &mut Response) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
}

#[handler]
async fn handle404(res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        res.render("Custom 404 Error Page");
        ctrl.skip_rest();
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(create_user()).await;
}

fn create_user() -> Service {
    let router = Router::new()
        .get(hello)
        .push(Router::with_path("500").get(error500));

    Service::new(router).catcher(Catcher::default().hoop(handle404))
}