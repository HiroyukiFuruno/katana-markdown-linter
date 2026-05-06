mod document;
mod protocol;
mod range;
mod server;

use std::path::PathBuf;

pub fn run_stdio() -> Result<(), String> {
    server::run_stdio()
}

pub(crate) fn uri_path(uri: &str) -> PathBuf {
    let path_str = if let Some(p) = uri.strip_prefix("file://") {
        if let Some(rest) = p.strip_prefix("localhost") {
            rest
        } else {
            p
        }
    } else {
        uri
    };

    let decoded = percent_encoding::percent_decode_str(path_str).decode_utf8_lossy();
    #[allow(unused_mut)]
    let mut path = decoded.as_ref();

    // On Windows, file:///c:/path/to/file should be c:\path\to\file.
    // The decoded path might start with /c:/...
    #[cfg(windows)]
    if path.starts_with('/') && path.chars().nth(2) == Some(':') {
        path = &path[1..];
    }

    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_path_parsing() {
        // Unix style
        assert_eq!(
            uri_path("file:///home/user/doc.md"),
            PathBuf::from("/home/user/doc.md")
        );

        // Localhost style
        assert_eq!(
            uri_path("file://localhost/home/user/doc.md"),
            PathBuf::from("/home/user/doc.md")
        );

        // Percent encoded
        assert_eq!(
            uri_path("file:///home/user/my%20doc.md"),
            PathBuf::from("/home/user/my doc.md")
        );

        // Windows style (on non-windows, it will keep the leading slash if it exists)
        // file:///C:/path -> /C:/path
        // file://localhost/C:/path -> /C:/path
        let win_uri = "file:///C:/path/to/file.md";
        let win_path = uri_path(win_uri);

        #[cfg(windows)]
        assert_eq!(win_path, PathBuf::from("C:/path/to/file.md"));

        #[cfg(not(windows))]
        assert_eq!(win_path, PathBuf::from("/C:/path/to/file.md"));
    }
}
