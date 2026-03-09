/*
 * Copyright (c) 2024-Present, Jeremy Plichta
 * Licensed under the MIT License
 */

//! Townhall - Tower-based REST control plane for Tinytown.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

use tinytown::{AppState, Town, create_router};

#[derive(Parser)]
#[command(
    name = "townhall",
    author,
    version,
    about = "Townhall - REST control plane for Tinytown"
)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    town: PathBuf,
    #[arg(short, long)]
    bind: Option<String>,
    #[arg(short, long)]
    port: Option<u16>,
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> tinytown::Result<()> {
    let cli = Cli::parse();
    let filter = if cli.verbose {
        EnvFilter::new("townhall=debug,tinytown=debug,tower_http=debug")
    } else {
        EnvFilter::new("townhall=info,tinytown=info")
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();
    let town = Town::connect(&cli.town).await?;
    let config = town.config().clone();
    info!("🏛️  Townhall starting for town: {}", config.name);
    let bind = cli
        .bind
        .clone()
        .unwrap_or_else(|| config.townhall.bind.clone());
    let port = cli.port.unwrap_or(config.townhall.rest_port);
    let addr: SocketAddr = format!("{}:{}", bind, port)
        .parse()
        .expect("Invalid address");
    let timeout_duration = Duration::from_millis(config.townhall.request_timeout_ms);
    let state = Arc::new(AppState { town });
    #[allow(deprecated)]
    let timeout_layer = TimeoutLayer::new(timeout_duration);
    let app = create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(timeout_layer)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    info!("🚀 Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
