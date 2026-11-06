use zed_extension_api::{self as zed, settings::LspSettings, Command, LanguageServerId, Worktree};

fn extract_kml_version(raw_version: &str) -> Option<(u8, u8)> {
    for token in raw_version.split_whitespace() {
        let normalized = token.trim_matches(|ch: char| !ch.is_ascii_digit() && ch != '.');
        let mut parts = normalized.split('.');

        let major: u8 = parts.next()?.parse().ok()?;
        let minor: u8 = parts.next()?.parse().ok()?;

        return Some((major, minor));
    }

    None
}

fn is_compatible_kml_version(raw_version: &str) -> bool {
    matches!(extract_kml_version(raw_version), Some((0, 19)))
}

struct KatanaMarkdownLinterExtension;

impl zed::Extension for KatanaMarkdownLinterExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Command> {
        let lsp_settings = LspSettings::for_worktree("kml", worktree)?;
        let mut executable_path = lsp_settings
            .binary
            .and_then(|binary| binary.path)
            .unwrap_or_else(|| "kml".to_string());

        if executable_path == "kml" {
            executable_path = worktree
                .which("kml")
                .ok_or_else(|| "kml executable not found in PATH".to_string())?;
        }

        warn_on_incompatible_version(&executable_path);

        Ok(Command {
            command: executable_path,
            args: vec!["lsp".to_string()],
            env: vec![],
        })
    }
}

fn warn_on_incompatible_version(executable_path: &str) {
    let mut command = zed::process::Command {
        command: executable_path.to_owned(),
        args: vec!["--version".to_string()],
        env: vec![],
    };

    match command.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let version = if stdout.trim().is_empty() {
                stderr.trim()
            } else {
                stdout.trim()
            };
            if version.is_empty() {
                eprintln!("Could not determine kml version output from kml --version");
            } else if !is_compatible_kml_version(version) {
                eprintln!(
                    "Warning: kml version may be incompatible with this extension. \"kml --version\" returned: {version}"
                );
            }
        }
        Err(err) => eprintln!("Failed to run kml --version: {err}"),
    }
}

zed::register_extension!(KatanaMarkdownLinterExtension);
