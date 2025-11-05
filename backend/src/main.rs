use std::net::SocketAddr;

use axum::Router;
use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::EnvFilter;
use funding_rate_backend::api::rest::routes::build_routes;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    let host = std::env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("API_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);

    let app = build_routes();

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    let listener = TcpListener::bind(&addr).await?;
    info!("Starting backend on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

