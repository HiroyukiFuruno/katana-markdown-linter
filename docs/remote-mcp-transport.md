# Remote MCP Transport

`kml-mcp-remote` is a self-hosted MCP Streamable HTTP server for clients that
cannot launch the local stdio server. It is separate from `kml-mcp` so remote
HTTP access does not inherit local workspace read or write permissions by
accident.

## Install

Build from a checkout:

    cargo build --bin kml-mcp-remote --features mcp-remote --locked

Install after publication:

    cargo install katana-markdown-linter --locked --features mcp-remote --bin kml-mcp-remote

Run the smoke test:

    make mcp-remote-smoke

## Run

    KML_MCP_REMOTE_TOKEN=change-me kml-mcp-remote

The default endpoint is:

    <http://127.0.0.1:3000/mcp>

The server uses MCP Streamable HTTP. Clients send JSON-RPC messages with HTTP
POST to the endpoint and must include:

- `Accept: application/json, text/event-stream`
- `Content-Type: application/json`
- `Authorization: Bearer <token>`
- `MCP-Protocol-Version: 2025-06-18` after initialization

The server runs in stateless JSON-response mode. It does not open a long-lived
SSE stream for server-initiated messages.

## Tools

| Tool | Workspace access | Description |
| --- | --- | --- |
| `check_text` | none | Returns structured diagnostics for Markdown content. |
| `fix_text` | none | Returns fixed content without writing files. |
| `config_validate` | none | Validates markdownlint-compatible JSON config. |
| `rule_list` | none | Lists known rule metadata. |
| `rule_get` | none | Returns metadata for one rule ID. |
| `rule_doc_get` | none | Returns Markdown documentation for one rule ID. |

These local stdio tools are intentionally absent from remote `tools/list` in
`v0.15.0`:

- `check_file`
- `check_directory`
- `fix_file_preview`
- `fix_file_apply`

Use `kml-mcp` over stdio when an agent needs workspace file access. Remote
workspace-backed mode is deferred until authenticated workspace assignment,
tenant boundary, explicit apply, and audit logging are implemented together.

## Environment

| Variable | Default | Meaning |
| --- | --- | --- |
| `KML_MCP_REMOTE_ADDR` | `127.0.0.1:3000` | Bind address. Keep localhost for local deployments. |
| `KML_MCP_REMOTE_ENDPOINT` | `/mcp` | Single MCP endpoint path. |
| `KML_MCP_REMOTE_TOKEN` | required | Bearer token required by default. |
| `KML_MCP_REMOTE_ALLOW_ANONYMOUS_READ` | `false` | Allows anonymous text-only operation only when set to `true`. |
| `KML_MCP_REMOTE_ALLOWED_HOSTS` | `localhost,127.0.0.1,::1` | Comma-separated Host header allowlist. |
| `KML_MCP_REMOTE_MAX_BODY_BYTES` | `1048576` | Maximum accepted HTTP request body size. |
| `KML_MCP_REMOTE_TIMEOUT_MS` | `30000` | Per-request timeout in milliseconds. |
| `KML_MCP_REMOTE_MAX_CONCURRENCY` | `16` | Maximum concurrent requests. |
| `RUST_LOG` | `info` | Logging filter. |

## Deployment Boundary

Terminate TLS in front of the server for non-local deployments. Set
`KML_MCP_REMOTE_ALLOWED_HOSTS` to the public hostnames accepted by that proxy and
keep bearer authentication enabled.

Do not present `kml-mcp` stdio, the MCPB bundle, or MCP Registry package
metadata as remote MCP transport. The Registry metadata continues to describe
only the local stdio MCPB package. `kml-mcp-remote` is a self-hosted endpoint
that operators deploy and protect themselves.

No public hosted service is included in `v0.15.0`. Operating a public endpoint
requires rate limiting, abuse handling, and tenant isolation outside the scope
of this release.
