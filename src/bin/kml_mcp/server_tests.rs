#[path = "server_tests/text.rs"]
mod text;
#[path = "server_tests/workspace.rs"]
mod workspace;

use super::*;
use rmcp::handler::server::wrapper::Parameters;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn tool_list_includes_workspace_tools_without_directory_apply() {
    let server = KmlMcpServer::new();
    let tool_names = server
        .tool_router
        .list_all()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();

    for expected in [
        "check_text",
        "fix_text",
        "config_validate",
        "rule_list",
        "rule_get",
        "check_file",
        "check_directory",
        "fix_file_preview",
        "fix_file_apply",
    ] {
        assert!(tool_names.contains(&expected.to_string()));
    }
    assert!(!tool_names.contains(&"fix_directory_apply".to_string()));
}

fn server_for_workspace(path: &std::path::Path) -> KmlMcpServer {
    KmlMcpServer::with_workspace(Workspace::new(path.to_path_buf()).unwrap())
}

fn temp_workspace(name: &str) -> std::path::PathBuf {
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("katana-markdown-linter-{name}-{id}"));
    std::fs::create_dir_all(&path).expect("temp workspace should be created");
    path
}

fn write_file(root: &std::path::Path, path: &str, content: &str) {
    let path = root.join(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dir should be created");
    }
    std::fs::write(path, content).expect("file should be written");
}
