use anyhow::Result;
use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use rmcp::transport::http::HttpServerTransport;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod kml_mcp;
use crate::kml_mcp::server::{KmlMcpServer, ServerMode};

#[tokio:main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let server = KmlMcpServer::new().with_mode(ServerMode::Remote);
    let transport = HttpServerTransport::new(server);

    let app = Router::new()
        .route("/sse", get(transport.sse_handler()).post(transport.post_handler()))
        .layer(middleware::from_fn(auth_middleware));

    tracing::info!("kml-mcp-remote listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn auth_middleware(req: Request, next: Next) -> Result<Response, Response> {
    let auth_token = env::var("KML_AUTH_TOKEN").ok();

    if let Some(token) = auth_token {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let expected = format!("Bearer {}", token);
        if auth_header != Some(&expected) {
            tracing::warn!("Unauthorized access attempt: Invalid or missing Bearer token");
            return Err(Response::builder()
                .status(axum::http::StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::from("401 Unauthorized: Invalid or missing Bearer token"))
                .unwrap());
        }
    }

    Ok(next.run(req).await)
}
