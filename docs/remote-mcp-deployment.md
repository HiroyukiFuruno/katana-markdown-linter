# Remote MCP Server Deployment

This document describes how to deploy and configure the `kml-mcp-remote` server for remote MCP access.

## Overview

`kml-mcp-remote` is a standalone binary that provides an MCP server reachable over HTTP (Streamable HTTP transport). It is designed for use cases where an API-hosted LLM needs to call `kml` tools directly.

## Deployment

### Prerequisites

- Rust toolchain (to build from source)
- `kml-mcp-remote` binary built with the `remote` feature.

### Building

```bash
cargo build --release --features remote --bin kml-mcp-remote
```

### Running

The server is configured via environment variables:

| Variable | Description | Default |
| --- | --- | --- |
| `PORT` | The port to listen on. | `3000` |
| `KML_WORKSPACE_ROOT` | The root directory for the workspace. | Current directory |
| `KML_AUTH_TOKEN` | Bearer token for authentication. | (None - insecure) |

Example:

```bash
export PORT=8080
export KML_AUTH_TOKEN=your-secure-token
export KML_WORKSPACE_ROOT=/path/to/your/workspace
./target/release/kml-mcp-remote
```

## Security Policy

### Authentication

`kml-mcp-remote` supports simple Bearer token authentication. If `KML_AUTH_TOKEN` is set, every request to `/mcp` must include an `Authorization: Bearer <token>` header.

**Note:** In production, you should always run the server behind a TLS-terminating reverse proxy (like Nginx or Caddy) to protect the authentication token in transit.

### Workspace Boundaries

In remote mode, workspace-backed tools (`check_file`, `check_directory`, `fix_file_preview`, `fix_file_apply`) are restricted by default to prevent unauthorized file system access. Only text-only tools and metadata tools are exposed:

- `check_text`
- `fix_text`
- `config_validate`
- `rule_list`
- `rule_get`

### DNS Rebinding Protection

The server only accepts requests with a `Host` header matching `localhost`, `127.0.0.1`, or `0.0.0.0` by default. If you are deploying to a specific domain, you may need to adjust the configuration in the source code or use a reverse proxy that normalizes the `Host` header.

## Connecting from MCP Clients

Use the HTTP transport to connect your MCP-compatible client:

- **Transport:** `http`
- **URL:** `http://<your-server-address>/mcp`
- **Auth:** Bearer token (if configured)
