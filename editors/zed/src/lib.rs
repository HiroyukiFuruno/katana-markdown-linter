use zed_extension_api::{self as zed, settings::LspSettings, Command, LanguageServerId, Worktree};

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

        Ok(Command {
            command: executable_path,
            args: vec!["lsp".to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(KatanaMarkdownLinterExtension);
