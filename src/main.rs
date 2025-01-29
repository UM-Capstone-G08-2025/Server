#![forbid(unsafe_code)]

mod config;
mod database;
mod error;
mod features;
mod middleware;
mod routes;
mod state;

use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use config::Config;
use state::AppState;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenvy::dotenv().context(".env file not found")?;

    // Configure logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug,server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new()?;

    let addr = SocketAddr::from((config.app.host, config.app.port));

    let state = AppState::new(config).await?;
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app = Router::new()
        .merge(routes::routes(state.clone()))
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_methods([http::Method::GET, http::Method::POST])
                    .allow_origin(Any),
            ),
        )
        .with_state(state)
        .into_make_service();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
