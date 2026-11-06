use super::*;

const INVALID_UTF8_PREFIX: [u8; 2] = [0xff, 0xfe];

#[tokio::test]
async fn file_tools_check_and_preview_inside_workspace() {
    let workspace = workspace_with_bad_heading("mcp-file-preview");
    let server = server_for_workspace(&workspace);

    let Json(check) = server
        .check_file(Parameters(FileRequest {
            path: "bad.md".to_string(),
            config_path: None,
            locale: Some("en".to_string()),
        }))
        .await
        .expect("check_file should succeed");
    assert_eq!(check.path, "bad.md");
    assert!(check.issue_count > 0);

    let Json(preview) = server
        .fix_file_preview(Parameters(FileRequest {
            path: "bad.md".to_string(),
            config_path: None,
            locale: None,
        }))
        .await
        .expect("preview should succeed");
    assert!(preview.changed);
    assert!(preview.diff.contains("-#Title"));
    assert!(preview.diff.contains("+# Title"));
    assert_eq!(
        std::fs::read_to_string(workspace.join("bad.md")).unwrap(),
        "#Title\n"
    );
}

#[tokio::test]
async fn apply_requires_opt_in_and_updates_workspace_file() {
    let workspace = workspace_with_bad_heading("mcp-file-apply");
    let server = server_for_workspace(&workspace);
    let request = |apply| {
        Parameters(FixFileApplyRequest {
            path: "bad.md".to_string(),
            config_path: None,
            locale: None,
            apply,
        })
    };

    let err = match server.fix_file_apply(request(false)).await {
        Ok(_) => panic!("apply flag should be required"),
        Err(message) => message,
    };
    assert!(err.contains("apply: true"));

    let Json(apply) = server
        .fix_file_apply(request(true))
        .await
        .expect("apply should succeed");
    assert!(apply.changed);
    assert_eq!(
        std::fs::read_to_string(workspace.join("bad.md")).unwrap(),
        "# Title\n"
    );
}

#[tokio::test]
async fn check_directory_respects_gitignore() {
    let workspace = temp_workspace("mcp-directory");
    write_file(&workspace, ".gitignore", "ignored.md\n");
    write_file(&workspace, "kept.md", "#Title\n");
    write_file(&workspace, "ignored.md", "#Title\n");
    let server = server_for_workspace(&workspace);

    let Json(response) = server
        .check_directory(Parameters(DirectoryRequest {
            path: ".".to_string(),
            config_path: None,
            locale: None,
            respect_gitignore: Some(true),
        }))
        .await
        .expect("check_directory should succeed");
    let paths = response
        .files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<Vec<_>>();
    assert!(paths.contains(&"kept.md"));
    assert!(!paths.contains(&"ignored.md"));
}

#[tokio::test]
async fn workspace_policy_rejects_parent_paths() {
    let workspace = temp_workspace("mcp-path-policy");
    write_file(&workspace, "bad.md", "#Title\n");
    let server = server_for_workspace(&workspace);

    let parent = workspace_path_error(&server, "../bad.md").await;
    assert!(parent.contains("inside the workspace root"));
}

#[cfg(unix)]
#[tokio::test]
async fn workspace_policy_rejects_symbolic_paths() {
    let workspace = temp_workspace("mcp-symlink-policy");
    write_file(&workspace, "bad.md", "#Title\n");
    let server = server_for_workspace(&workspace);
    std::os::unix::fs::symlink(workspace.join("bad.md"), workspace.join("link.md"))
        .expect("symlink should be created");
    let error = workspace_path_error(&server, "link.md").await;
    assert!(error.contains("Symbolic") || error.contains("symbolic"));
}

async fn workspace_path_error(server: &KmlMcpServer, path: &str) -> String {
    match server
        .check_file(Parameters(FileRequest {
            path: path.to_string(),
            config_path: None,
            locale: None,
        }))
        .await
    {
        Ok(_) => panic!("forbidden workspace path was accepted: {path}"),
        Err(message) => message,
    }
}

#[tokio::test]
async fn check_file_reports_non_utf8_file_as_error() {
    let workspace = temp_workspace("mcp-non-utf8");
    std::fs::write(workspace.join("binary.md"), INVALID_UTF8_PREFIX)
        .expect("binary fixture should be written");
    let server = server_for_workspace(&workspace);

    let message = match server
        .check_file(Parameters(FileRequest {
            path: "binary.md".to_string(),
            config_path: None,
            locale: None,
        }))
        .await
    {
        Ok(_) => panic!("non UTF-8 Markdown should fail"),
        Err(message) => message,
    };

    assert!(message.contains("not UTF-8") || message.contains("could not be read"));
}

fn workspace_with_bad_heading(name: &str) -> std::path::PathBuf {
    let workspace = temp_workspace(name);
    write_file(&workspace, "bad.md", "#Title\n");
    write_file(&workspace, ".markdownlint.json", "{ \"default\": true }\n");
    workspace
}
