# MCP Server

`kml-mcp` is an optional MCP adapter for `katana-markdown-linter`. It exists
for agents and editors that want structured diagnostics, rule metadata, and
workspace-scoped file checks without invoking the `kml` CLI themselves.

## Install

    cargo build --bin kml-mcp --features mcp --locked

For local smoke testing:

    cargo install --path . --locked --features mcp --bin kml-mcp --root ./target/mcp-install

After publication, install from crates.io:

    cargo install katana-markdown-linter --locked --features mcp --bin kml-mcp

Run the local stdio smoke test:

    make mcp-stdio-smoke

## Run

    kml-mcp --workspace-root /absolute/path/to/workspace

The server uses MCP stdio transport. If `--workspace-root` is omitted, the
current working directory is used.

## Tools

| Tool | Workspace access | Description |
| --- | --- | --- |
| `check_text` | none | Returns structured diagnostics for Markdown content. |
| `fix_text` | none | Returns fixed content without writing files. |
| `config_validate` | none | Validates markdownlint-compatible JSON config. |
| `rule_list` | none | Lists known rule metadata. |
| `rule_get` | none | Returns metadata for one rule ID. |
| `check_file` | read | Checks one Markdown file under the workspace root. |
| `check_directory` | read | Recursively checks Markdown files under one workspace directory. |
| `fix_file_preview` | read | Previews safe fixes for one workspace file and returns a text diff. |
| `fix_file_apply` | write | Applies safe fixes to one workspace file only when `apply` is `true`. |

All tools default to English. Pass an optional `locale` string to request
localized messages or rule descriptions. Supported values currently resolve to
English (`en`, `en-US`) or Japanese (`ja`, `ja-JP`); unsupported MCP locale
values fall back to English instead of failing the tool call.

### Text Tools

    { "content": "# title\n\n### skipped\n", "locale": "ja-JP" }

    { "rule_id": "MD003", "locale": "ja" }

Localized responses keep stable fields such as `message_id`, `message_params`,
`kind`, `expected`, `actual`, `allowed`, rule IDs, and documentation URLs.

### Workspace Tools

All workspace paths must be relative to the configured workspace root.

    { "path": "README.md", "locale": "en" }

    { "path": "docs", "respect_gitignore": true }

    { "path": "README.md", "config_path": ".markdownlint.json" }

`fix_file_preview` returns a unified text diff and never writes files.
`fix_file_apply` writes only when the request includes `apply: true`.

    { "path": "README.md", "apply": true }

## Safety Boundary

Workspace tools enforce these boundaries:

- the workspace root is resolved once at server startup
- file and directory inputs must be relative paths
- `..` traversal and absolute paths are rejected
- symbolic path components are rejected
- directory checks do not traverse symbolic paths
- `.gitignore`, global gitignore, and git exclude files are respected by default
- non-UTF-8 Markdown files fail with a structured tool error
- directory apply is not exposed

File mutation is intentionally limited to `fix_file_apply` with `apply: true`.
Agents should call `fix_file_preview` first when they need a diff before
writing.

The core library remains MCP-free. `kml-mcp` is an adapter that depends on the
public Rust API.

## Client Configuration

Use an absolute workspace path in shared configuration. The examples below use
`/absolute/path/to/workspace` as a placeholder.

### Codex

Add the server to `~/.codex/config.toml`:

    [mcp_servers.kml]
    command = "kml-mcp"
    args = ["--workspace-root", "/absolute/path/to/workspace"]
    default_tools_approval_mode = "prompt"

### Claude Code

Register a local stdio server:

    claude mcp add --transport stdio --scope project kml -- \
      kml-mcp --workspace-root /absolute/path/to/workspace

Equivalent JSON form:

    claude mcp add-json kml \
      '{"type":"stdio","command":"kml-mcp","args":["--workspace-root","/absolute/path/to/workspace"]}'

### Antigravity

Open the Agent Panel, choose the MCP server manager, then edit the raw MCP
configuration. Add:

    {
      "mcpServers": {
        "kml": {
          "command": "kml-mcp",
          "args": ["--workspace-root", "/absolute/path/to/workspace"]
        }
      }
    }

## MCP Registry Metadata (Draft)

This is a draft of `server.json` for registration in the [MCP Registry](https://modelcontextprotocol.io/registry/about).

```json
{
  "$schema": "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json",
  "name": "io.github.HiroyukiFuruno/kml",
  "title": "KatanA Markdown Linter",
  "description": "Enterprise-grade Markdown linter and formatter with workspace safety.",
  "repository": {
    "url": "https://github.com/HiroyukiFuruno/katana-markdown-linter",
    "source": "github"
  },
  "version": "0.13.0",
  "packages": [
    {
      "registryType": "mcpb",
      "identifier": "https://github.com/HiroyukiFuruno/katana-markdown-linter/releases/download/v0.13.0/kml-mcp.mcpb",
      "fileSha256": "TODO_SHA256_DURING_RELEASE",
      "transport": {
        "type": "stdio"
      }
    },
    {
      "registryType": "oci",
      "identifier": "ghcr.io/hiroyukifuruno/kml-mcp:0.13.0",
      "transport": {
        "type": "stdio"
      }
    }
  ]
}
```

### Security Review Checklist for Registry

- [ ] Workspace root enforcement is active and tested.
- [ ] Symlink traversal is disabled by default.
- [ ] `fix_file_apply` requires explicit `apply: true`.
- [ ] Remote transport (SSE) is NOT included in this metadata.
- [ ] Documentation URL points to the Safety Boundary section of this document.
