use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = "0.0.0.0:8080"; 
    let listner = TcpListener::bind(addr).await?;
    info!("Starting server on {}", addr);
    loop {
        let (stream, addr) = listner.accept().await?;
        info!("Accepted connection from {}", addr);
        tokio::spawn(async move {
            let _ = handle_client(stream).await;
        });
    }
    Ok(())
}