use std::path::{Component, Path, PathBuf};

pub(super) fn clean_relative_path(value: &str) -> Result<PathBuf, String> {
    let path = Path::new(value);
    if path.as_os_str().is_empty() {
        return Err("workspace path must not be empty".to_string());
    }
    if path.is_absolute() {
        return Err("workspace path must be relative".to_string());
    }

    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => clean.push(value),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err("workspace path must stay inside the workspace root".to_string());
            }
        }
    }
    if clean.as_os_str().is_empty() {
        Ok(PathBuf::from("."))
    } else {
        Ok(clean)
    }
}

pub(super) fn reject_symlink_components(root: &Path, relative: &Path) -> Result<(), String> {
    let mut current = root.to_path_buf();
    for component in relative.components() {
        if let Component::Normal(value) = component {
            current.push(value);
            if is_symlink(&current) {
                return Err(format!(
                    "symbolic paths are not allowed: {}",
                    relative.display()
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn read_markdown_file(path: &Path) -> Result<String, String> {
    if !path.is_file() {
        return Err(format!("not a file: {}", path.display()));
    }
    std::fs::read_to_string(path)
        .map_err(|error| format!("file is not UTF-8 or could not be read: {error}"))
}

fn is_symlink(path: &Path) -> bool {
    std::fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
}
