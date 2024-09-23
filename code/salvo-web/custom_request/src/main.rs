use salvo::prelude::*;
use serde::{Deserialize, Serialize};


// 默认从请求正文提取数据
#[derive(Serialize, Deserialize, Extractible, Debug)]
#[salvo(extract(default_source(from = "body")))]
struct GoodMan<'a> {
    // 尝试从URL的路径参数提取id字段的值
    #[salvo(extract(source(from="param")))]
    id: i64,

    username: &'a str,

    first_name: String,
    last_name: String,

    lovers: Vec<String>,
}

#[handler]
async fn home(res: &mut Response) {
    res.render(Text::Html("<html><body>hello</body><html>"));
}

#[handler]
async fn set_status_code(res: &mut Response) {
    res.status_code(StatusCode::ACCEPTED);
}

// 两种方法解析
#[handler]
async fn edit(req: &mut Request, res: &mut Response) {
    let good_man: GoodMan<'_> = req.extract().await.unwrap();
    res.render(Json(good_man));
}

#[handler]
async fn edit_salvo<'a>(good_man: GoodMan<'a> ,res: &mut Response) {
    res.render(Json(good_man));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::with_hoop(set_status_code).get(home).push(Router::with_path("user/<id:num>").post(edit));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(router).await;
}
