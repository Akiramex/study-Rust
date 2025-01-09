use salvo::prelude::*;
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan}, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() {
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(console).init();

    let addr = "0.0.0.0:5800";
    let acceptor = TcpListener::new(addr).bind().await;
    let router = Router::new().get(hello);
    info!("Starting server on {}", addr);
    Server::new(acceptor).serve(router).await;
}

#[instrument]
#[handler]
async fn hello() -> &'static str {
    "Hello world"
}