#[path = "kml_mcp/model.rs"]
mod model;
#[path = "kml_mcp/server.rs"]
mod server;
#[path = "kml_mcp/workspace.rs"]
mod workspace;

use crate::server::{KmlMcpServer, ServerMode};
use crate::workspace::Workspace;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    Router,
};
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;
    let workspace_root = std::env::var("KML_WORKSPACE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().expect("current dir should be available"));
    let auth_token = std::env::var("KML_AUTH_TOKEN").ok();

    if auth_token.is_none() {
        eprintln!(
            "Warning: KML_AUTH_TOKEN is not set. The server will not enforce authentication."
        );
    }

    let workspace = Workspace::new(workspace_root)?;
    let server_proto = KmlMcpServer::with_workspace(workspace).with_mode(ServerMode::Remote);

    let session_manager = Arc::new(LocalSessionManager::default());
    let config = StreamableHttpServerConfig::default().with_allowed_hosts(vec![
        "localhost".to_string(),
        format!("127.0.0.1:{}", port),
        format!("0.0.0.0:{}", port),
    ]);

    let service_factory = move || Ok(server_proto.clone());
    let transport = StreamableHttpService::new(service_factory, session_manager, config);

    let app = Router::new()
        .fallback_service(transport)
        .layer(middleware::from_fn(move |req, next| {
            auth_middleware(req, next, auth_token.clone())
        }))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Remote MCP server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn auth_middleware(
    req: Request<Body>,
    next: Next,
    expected_token: Option<String>,
) -> Result<Response, StatusCode> {
    if let Some(token) = expected_token {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let authenticated = match auth_header {
            Some(h) if h.starts_with("Bearer ") => h[7..] == *token,
            _ => false,
        };

        if !authenticated {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    Ok(next.run(req).await)
}
