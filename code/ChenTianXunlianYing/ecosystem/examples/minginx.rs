use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::net::{
    TcpListener,
    TcpStream,
};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    listen_addr: String,
    upstream_addr: String,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = fmt::Layer::new()
        .pretty()
        .with_filter(LevelFilter::DEBUG);
    tracing_subscriber::registry().with(layer).init();

    let config = resolve_config();
    let config = Arc::new(config);
    
    info!("{config:?}");

    let listener = TcpListener::bind(&config.listen_addr).await?;
    
    loop {
        let config = Arc::clone(&config);
        let (client, addr) = listener.accept().await?;
        info!("Accept connection from: {}", addr);
        tokio::spawn(async move {
            let upstream = TcpStream::connect(&config.upstream_addr).await?;
            proxy(client, upstream).await?;
            Ok::<(), anyhow::Error>(())
        });
    }
}

async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> anyhow::Result<()> {
    let (mut client_reader, mut client_writer) = client.split();
    let (mut upstream_reader, mut upstream_writer) = upstream.split();
    
    let client_to_upstream = tokio::io::copy(&mut client_reader, &mut upstream_writer);
    let upstream_to_client = tokio::io::copy(&mut upstream_reader, &mut client_writer);

    if let Err(e) = tokio::try_join!(client_to_upstream, upstream_to_client) {
        warn!("Error: {e}")
    }
    Ok(())
}

fn resolve_config() -> Config {
    Config {
        listen_addr: "0.0.0.0:8081".to_string(),
        upstream_addr: "0.0.0.0:8080".to_string(),
    }
}