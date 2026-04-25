# MCP Server

`kml-mcp` is an optional experimental MCP adapter for `katana-markdown-linter`.
It exists for agents and editors that want structured diagnostics and rule
metadata without using the CLI as a subprocess.

## Build

~~~bash
cargo build --bin kml-mcp --features mcp --locked
~~~

## Run

~~~bash
cargo run --bin kml-mcp --features mcp --locked
~~~

The server uses MCP stdio transport.

## Tools

| Tool | Workspace access | Description |
| --- | --- | --- |
| `check_text` | none | Returns structured diagnostics for Markdown content. |
| `fix_text` | none | Returns fixed content without writing files. |
| `config_validate` | none | Validates markdownlint-compatible JSON config. |
| `rule_list` | none | Lists known rule metadata. |
| `rule_get` | none | Returns metadata for one rule ID. |

All tools default to English. Pass an optional `locale` string to request
localized messages or rule descriptions. Supported values currently resolve to
English (`en`, `en-US`) or Japanese (`ja`, `ja-JP`); unsupported MCP locale
values fall back to English instead of failing the tool call.

Examples:

~~~json
{ "content": "# title\n\n### skipped\n", "locale": "ja-JP" }
~~~

~~~json
{ "rule_id": "MD003", "locale": "ja" }
~~~

Localized responses keep stable fields such as `message_id`, `message_params`,
`kind`, `expected`, `actual`, `allowed`, rule IDs, and documentation URLs.

## Safety Boundary

The prototype does not expose file read or file write tools.

Future file-oriented MCP tools must add:

- explicit opt-in
- workspace-root path allowlist
- dry-run diff before write
- structured reporting of changed paths

The core library remains MCP-free. `kml-mcp` is an adapter that depends on the
public Rust API.
