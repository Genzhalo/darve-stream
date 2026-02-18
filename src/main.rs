mod config;
mod interfaces;
mod models;
mod routes;
mod services;
mod state;
mod views;

use std::net::{Ipv4Addr, SocketAddr};

use axum::Router;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::{config::Config, state::AppState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // set true in production (HTTPS)
        .with_http_only(true);

    let state = AppState::new(config);

    let app = Router::new()
        .merge(routes::router())
        .layer(session_layer)
        .with_state(state);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3001));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
