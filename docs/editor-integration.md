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

Add schema mapping to `.vscode/settings.json`:

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
files. LSP support requires a VS Code extension or generic LSP bridge that can
launch `kml lsp` over stdio.

## Zed

Add schema mapping to `.zed/settings.json`:

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
Running `kml lsp` for Markdown files requires a Zed extension or another adapter
that registers `kml` as a Markdown language server.

## Local Schema Fallback

If you prefer to keep the schema file within your repository for offline use:

1. Generate the schema: `kml config schema > .markdownlint.schema.json`
2. Reference the local file in your editor settings (e.g., using a relative path).

Note: Remember to regenerate the local schema file after upgrading `kml`.

## Neovim

Use Neovim's built-in LSP client when `kml` is installed on `PATH`:

~~~lua
vim.lsp.config("kml", {
  cmd = { "kml", "lsp" },
  filetypes = { "markdown" },
  root_markers = { ".markdownlint.json", ".markdownlint.jsonc", ".git" },
})

vim.lsp.enable("kml")
~~~

After opening a Markdown buffer, run `:checkhealth vim.lsp` to confirm the
client attached. The server reports diagnostics as documents open or change and
returns formatting edits through the normal LSP formatting command.
