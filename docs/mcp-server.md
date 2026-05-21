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

After `v0.19.3`, launch the same server through official wrappers without a
Rust toolchain:

    npx --yes katana-markdown-linter@0.19.3 kml-mcp --workspace-root /absolute/path/to/workspace
    bunx --package katana-markdown-linter@0.19.3 kml-mcp --workspace-root /absolute/path/to/workspace
    uvx --from katana-markdown-linter==0.19.3 kml-mcp --workspace-root /absolute/path/to/workspace

Run the local stdio smoke test:

    just mcp-stdio-smoke

Build and smoke test the MCPB bundle:

    just VERSION=v0.19.3 mcpb-smoke

## Run

    kml-mcp --workspace-root /absolute/path/to/workspace

The server uses MCP stdio transport. If `--workspace-root` is omitted, the
current working directory is used.

For API-hosted LLM clients that need an HTTP-reachable MCP endpoint, use the
separate `kml-mcp-remote` binary documented in
[`remote-mcp-transport.md`](remote-mcp-transport.md). The local stdio server and
the remote server are intentionally separate because workspace file access,
authentication, and write policy have different safety boundaries over HTTP.

## Tools

| Tool | Workspace access | Description |
| --- | --- | --- |
| `check_text` | none | Returns structured diagnostics for Markdown content. |
| `fix_text` | none | Returns fixed content without writing files. |
| `config_validate` | none | Validates markdownlint-compatible JSON config against the kml schema model. |
| `rule_list` | none | Lists known rule metadata. |
| `rule_get` | none | Returns metadata for one rule ID. |
| `rule_doc_get` | none | Returns Markdown documentation for one rule ID. |
| `check_file` | read | Checks one Markdown file under the workspace root. |
| `check_directory` | read | Recursively checks Markdown files under one workspace directory. |
| `fix_file_preview` | read | Previews safe fixes for one workspace file and returns a text diff. |
| `fix_file_apply` | write | Applies safe fixes to one workspace file only when `apply` is `true`. |

All tools default to English. Pass an optional `locale` string to request
localized messages, rule descriptions, or rule Markdown documentation.
Supported values resolve to `en`, `ja`, `zh-CN`, `zh-TW`, `ko`, `pt`, `fr`,
`de`, `es`, and `it`, including common region forms such as `fr-FR`,
`pt-BR`, or `ko-KR`. Unsupported MCP locale values fall back to English
instead of failing the tool call.

### Text Tools

    { "content": "# title\n\n### skipped\n", "locale": "ja-JP" }

    { "rule_id": "MD003", "locale": "ja" }

    { "rule_id": "MD013", "locale": "fr-FR" }

Localized responses keep stable fields such as `message_id`, `message_params`,
`kind`, `expected`, `actual`, `allowed`, rule IDs, and documentation URLs.
`config_validate` uses the same schema-derived rule metadata as
`kml config schema`.

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
    command = "npx"
    args = ["--yes", "katana-markdown-linter@0.19.3", "kml-mcp", "--workspace-root", "/absolute/path/to/workspace"]
    default_tools_approval_mode = "prompt"

`uvx` is also valid:

    [mcp_servers.kml]
    command = "uvx"
    args = ["--from", "katana-markdown-linter==0.19.3", "kml-mcp", "--workspace-root", "/absolute/path/to/workspace"]
    default_tools_approval_mode = "prompt"

### Claude Code

Register a local stdio server:

    claude mcp add --transport stdio --scope project kml -- \
      npx --yes katana-markdown-linter@0.19.3 kml-mcp --workspace-root /absolute/path/to/workspace

Equivalent JSON form:

    claude mcp add-json kml \
      '{"type":"stdio","command":"npx","args":["--yes","katana-markdown-linter@0.19.3","kml-mcp","--workspace-root","/absolute/path/to/workspace"]}'

### Antigravity

Open the Agent Panel, choose the MCP server manager, then edit the raw MCP
configuration. Add:

    {
      "mcpServers": {
        "kml": {
          "command": "npx",
          "args": ["--yes", "katana-markdown-linter@0.19.3", "kml-mcp", "--workspace-root", "/absolute/path/to/workspace"]
        }
      }
    }

## MCPB And Registry Metadata

From `v0.14.0`, the local stdio server is published as an MCPB bundle attached
to each GitHub Release:

    katana-markdown-linter-0.19.3.mcpb
    katana-markdown-linter-0.19.3.mcpb.sha256

The committed `server.json` is the source metadata. During release,
`just VERSION=v0.19.3 mcp-server-json` renders `target/mcpb/server.json` with
the final GitHub Release artifact URL and computed `fileSha256` value. The
rendered file is the MCP Registry publication input.

Validate the rendered metadata:

    just VERSION=v0.19.3 server-json-validate

The MCP Registry server name is `io.github.HiroyukiFuruno/kml`. The metadata
uses only a package-based stdio transport and does not declare remote MCP
transport.

### Registry Security Checklist

- Workspace root enforcement is active and tested.
- Absolute paths and parent-directory traversal are rejected.
- Symbolic path components are rejected.
- Directory checks respect git ignore files by default.
- `fix_file_preview` is the non-mutating path for file fixes.
- `fix_file_apply` writes only when `apply` is `true`.
- Directory-wide apply is not exposed.
- Remote MCP transport is not claimed by this metadata.
