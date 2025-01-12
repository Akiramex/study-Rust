use salvo::prelude::*;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tracing::{info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan}, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("log", "ecosystem.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);    

    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);


    let file = fmt::Layer::new()
        .with_ansi(false)
        .with_writer(non_blocking)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        .init();

    let addr = "0.0.0.0:5800";
    let acceptor = TcpListener::new(addr).bind().await;
    let router = Router::new().get(hello);
    info!("Starting server on {}", addr);
    Server::new(acceptor).serve(router).await;
}

#[instrument]
#[handler]
async fn hello() -> &'static str {
    let a = 100;
    warn!(target: "input_events", abc = a, "Hello");
    hello2().await;
    "Hello world"
}

async fn hello2() -> anyhow::Result<()>{
    let mut file = File::create("foo.txt").await?;
    file.write_all(b"hello, worldasd!").await?;
    Ok(())
}