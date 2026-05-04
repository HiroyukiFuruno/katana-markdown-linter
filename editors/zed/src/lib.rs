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
    matches!(extract_kml_version(raw_version), Some((0, 18)))
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

        let version_output = zed_extension_api::process::Command::new(&executable_path)
            .arg("--version")
            .output();

        if let Ok(output) = version_output {
            let version_text = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            let stderr_text = String::from_utf8_lossy(&output.stderr)
                .trim()
                .to_string();
            let effective_version = if !version_text.is_empty() {
                version_text
            } else {
                stderr_text
            };

            if !effective_version.is_empty() && !is_compatible_kml_version(&effective_version) {
                eprintln!(
                    "Warning: kml version may be incompatible with this extension. \"kml --version\" returned: {}",
                    effective_version
                );
            } else if effective_version.is_empty() {
                eprintln!("Could not determine kml version output from kml --version");
            }
        } else if let Err(err) = version_output {
            eprintln!("Failed to run kml --version: {err}");
        }

        Ok(Command {
            command: executable_path,
            args: vec!["lsp".to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(KatanaMarkdownLinterExtension);
