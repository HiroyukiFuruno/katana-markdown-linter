mod document;
mod protocol;
mod range;
mod server;

use std::path::PathBuf;

pub struct LspServerRunner;

impl LspServerRunner {
    pub fn run_stdio() -> Result<(), String> {
        server::LspServerRunner::run_stdio()
    }
}

pub(crate) struct LspUri;

impl LspUri {
    pub(crate) fn path(uri: &str) -> PathBuf {
        let path_str = if let Some(path) = uri.strip_prefix("file://") {
            if let Some(rest) = path.strip_prefix("localhost") {
                rest
            } else {
                path
            }
        } else {
            uri
        };

        let decoded = percent_encoding::percent_decode_str(path_str).decode_utf8_lossy();
        #[allow(unused_mut)]
        let mut path = decoded.as_ref();

        #[cfg(windows)]
        if path.starts_with('/') && path.chars().nth(2) == Some(':') {
            path = &path[1..];
        }

        PathBuf::from(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_path_parsing() {
        assert_eq!(
            LspUri::path("file:///home/user/doc.md"),
            PathBuf::from("/home/user/doc.md")
        );

        assert_eq!(
            LspUri::path("file://localhost/home/user/doc.md"),
            PathBuf::from("/home/user/doc.md")
        );

        assert_eq!(
            LspUri::path("file:///home/user/my%20doc.md"),
            PathBuf::from("/home/user/my doc.md")
        );

        let win_uri = "file:///C:/path/to/file.md";
        let win_path = LspUri::path(win_uri);

        #[cfg(windows)]
        assert_eq!(win_path, PathBuf::from("C:/path/to/file.md"));

        #[cfg(not(windows))]
        assert_eq!(win_path, PathBuf::from("/C:/path/to/file.md"));
    }
}
