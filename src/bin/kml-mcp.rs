#[path = "kml_mcp/model.rs"]
mod model;
#[path = "kml_mcp/server.rs"]
mod server;
#[path = "kml_mcp/shared.rs"]
mod shared;
#[path = "kml_mcp/workspace.rs"]
mod workspace;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::KmlMcpRuntime::run_from_env().await
}
