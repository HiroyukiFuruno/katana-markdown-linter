#[path = "kml_mcp/remote_model.rs"]
mod model;
#[path = "kml_mcp/remote.rs"]
mod remote;
#[path = "kml_mcp/shared.rs"]
mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    remote::run_from_env().await
}
