use super::KmlMcpServer;
use crate::workspace::Workspace;
use rmcp::ServiceExt;
use std::path::PathBuf;

pub(crate) struct KmlMcpRuntime;

impl KmlMcpRuntime {
    pub(crate) async fn run_from_env() -> Result<(), Box<dyn std::error::Error>> {
        let workspace = Workspace::new(parse_workspace_root()?)?;
        let service = KmlMcpServer::with_workspace(workspace)
            .serve(rmcp::transport::stdio())
            .await?;
        service.waiting().await?;
        Ok(())
    }
}

fn parse_workspace_root() -> Result<PathBuf, String> {
    let mut args = std::env::args().skip(1);
    let mut root = std::env::current_dir().map_err(|error| error.to_string())?;
    while let Some(arg) = args.next() {
        if arg == "--workspace-root" {
            let value = args
                .next()
                .ok_or_else(|| "--workspace-root requires a path".to_string())?;
            root = PathBuf::from(value);
        }
    }
    Ok(root)
}
