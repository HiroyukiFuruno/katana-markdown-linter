# Remote MCP Deployment

This document describes how to deploy `kml-mcp-remote` as a standalone MCP server over HTTP/SSE.

## Features

- **SSE Transport**: Provides standard Model Context Protocol over Server-Sent Events.
- **Authentication**: Bearer token authentication via `KML_AUTH_TOKEN`.
- **Security**: Restricted toolset that prevents arbitrary workspace file access (406 errors for `check_file`, `fix_file_apply`, etc.).

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Listening port for the HTTP server | `3000` |
| `KML_AUTH_TOKEN` | Required Bearer token for authentication | (None, open if unset) |
| `RUST_LOG` | Logging level (`info`, `debug`, etc.) | `info` |

## Deployment with Docker

```bash
docker build -t kml-mcp-remote -f Dockerfile.remote .
docker run -p 3000:3000 -e KML_AUTH_TOKEN=your-secret-token kml-mcp-remote
```

## Connecting from MCP Clients

Configure your MCP client (e.g., Claude Desktop, Zed) to connect to `http://<host>:<port>/sse`.

If authentication is enabled, ensure the client sends the `Authorization: Bearer <token>` header during the initial SSE GET request and subsequent POST requests.
