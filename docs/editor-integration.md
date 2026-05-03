# Editor Integration

`kml` exposes two editor-facing entry points:

- `kml config schema` prints the JSON Schema for `.markdownlint.json` and
  `.markdownlint.jsonc`.
- `kml lsp` starts a stdio Language Server Protocol server for Markdown
  diagnostics, formatting, range formatting, and safe quick fixes.

## Configuration Schema

The stable schema ID is:

<https://schemas.katana.tools/kml/markdownlint.schema.json>

Versioned schemas are available as release assets for stable dependency pinning.
For example, `v0.18.0` is available at:

<https://github.com/HiroyukiFuruno/katana-markdown-linter/releases/download/v0.18.0/markdownlint.schema.v0.18.0.json>

### Compatibility Policy

The `kml` configuration schema follows an **additive-first** compatibility
policy. Existing rule properties, types, enum values, and defaults are
preserved within a minor release line. New rules and properties are added as
additive changes that do not break existing configuration files.

Breaking schema changes are only introduced in major releases or with explicit
deprecation notices in the release notes.

### Local Fallback

Generate a local copy when an editor cannot fetch remote schemas:

~~~bash
kml config schema > schema/markdownlint.schema.json
~~~

The schema is generated from the same rule metadata used by `kml rule` and
configuration validation. Rule entries accept either a boolean or an object with
`enabled` plus documented rule properties.

## VS Code

### Official Extension (Recommended)

The **KatanA Markdown Linter** extension is the easiest way to use `kml` in
VS Code. It automatically:

- Launches `kml lsp` when you open a Markdown file.
- Associates `.markdownlint.json` and `.markdownlint.jsonc` with the published
  JSON schema.
- Provides diagnostics, formatting, and safe quick fixes.

**Installation:**
Search for "KatanA Markdown Linter" in the VS Code Marketplace, or sideload
from the repository:

~~~bash
cd editors/vscode
npm install
npm run compile
code --extensionDevelopmentPath=$PWD
~~~

**Configuration:**

- `kml.executablePath`: Path to the `kml` binary. Defaults to `kml` (on PATH).

### Manual Configuration

If you do not want to install the extension, add schema mapping to
`.vscode/settings.json`:

~~~json
{
  "files.associations": {
    ".markdownlint.jsonc": "jsonc"
  },
  "json.schemas": [
    {
      "fileMatch": [
        "**/.markdownlint.json",
        "**/.markdownlint.jsonc"
      ],
      "url": "https://schemas.katana.tools/kml/markdownlint.schema.json"
    }
  ]
}
~~~

VS Code uses this for completion, hover text, and validation in configuration
files. LSP support requires a generic LSP bridge plugin that can launch
`kml lsp` over stdio.

## Zed

### Official Extension (Recommended)

Sideload the extension from the repository:

1. Open Zed.
2. Run the `zed: install dev extension` action.
3. Select the `editors/zed` directory.

The extension automatically registers `kml lsp` for Markdown files and supports
the following configuration:

- `lsp`:
  - `kml`:
    - `binary`:
      - `path`: Path to the `kml` binary. Defaults to `kml` (on PATH).

### Manual Configuration

If you do not want to install the extension, add schema mapping to
`.zed/settings.json`:

~~~json
{
  "lsp": {
    "json-language-server": {
      "settings": {
        "json": {
          "schemas": [
            {
              "fileMatch": [
                "**/.markdownlint.json",
                "**/.markdownlint.jsonc"
              ],
              "url": "https://schemas.katana.tools/kml/markdownlint.schema.json"
            }
          ]
        }
      }
    }
  }
}
~~~

Zed can use the JSON language server for schema-backed configuration editing.

## Local Schema Fallback

If you prefer to keep the schema file within your repository for offline use:

1. Generate the schema: `kml config schema > .markdownlint.schema.json`
2. Reference the local file in your editor settings (e.g., using a relative path).

Note: Remember to regenerate the local schema file after upgrading `kml`.

## Neovim

Use Neovim's built-in LSP client when `kml` is installed on `PATH`:

~~~lua
-- Sample configuration using nvim-lspconfig
local configs = require('lspconfig.configs')
local nvim_lsp = require('lspconfig')

if not configs.kml then
  configs.kml = {
    default_config = {
      cmd = { 'kml', 'lsp' },
      filetypes = { 'markdown' },
      root_dir = nvim_lsp.util.root_pattern('.markdownlint.json', '.markdownlint.jsonc', '.git'),
      settings = {},
    },
  }
end

nvim_lsp.kml.setup({})
~~~

After opening a Markdown buffer, run `:checkhealth lsp` or `:LspInfo` to confirm
the client attached. The server reports diagnostics as documents open or change
and returns formatting edits through the normal LSP formatting command.

## Troubleshooting

### Binary Path and Version

If the editor fails to start the `kml` server, check the following:

- **PATH**: Ensure `kml` is available on your system `PATH`.
- **Custom Path**: Use `kml.executablePath` (VS Code) or Zed LSP binary settings to point to the exact location of the binary.
- **Compatibility**: The extension checks for a compatible `kml` version (e.g., `^0.18.0`). If you are using an older version, the extension may show a warning. Upgrade `kml` or the extension to resolve this.

### Workspace Root (VS Code)

When using relative paths for `kml.executablePath`, the extension resolves them
relative to the first workspace folder. Ensure your project is opened as a
workspace folder if you rely on relative paths.
