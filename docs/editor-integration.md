# Editor Integration

`kml` exposes two editor-facing entry points:

- `kml config schema` prints the JSON Schema for `.markdownlint.json` and
  `.markdownlint.jsonc`.
- `kml lsp` starts a stdio Language Server Protocol server for Markdown
  diagnostics, formatting, range formatting, and safe quick fixes.

## Configuration Schema

The stable schema ID is:

<https://schemas.katana.tools/kml/markdownlint.schema.json>

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
