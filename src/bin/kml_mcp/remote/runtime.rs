use super::{
    config::RemoteConfig,
    transport::{handle_mcp, RemoteHttpState},
};
use axum::{routing::any, Router};
use std::io;

pub(crate) async fn run_from_env() -> Result<(), Box<dyn std::error::Error>> {
    let config = RemoteConfig::from_env().map_err(invalid_input)?;
    init_logging();

    let state = RemoteHttpState::new(
        config.auth.clone(),
        config.limits.clone(),
        config.allowed_hosts.clone(),
    );
    let app = Router::new()
        .route(&config.endpoint, any(handle_mcp))
        .with_state(state);

    eprintln!(
        "kml-mcp-remote listening on http://{}{}",
        config.addr, config.endpoint
    );
    let listener = tokio::net::TcpListener::bind(config.addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn init_logging() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(io::stderr)
        .try_init();
}

fn invalid_input(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}
