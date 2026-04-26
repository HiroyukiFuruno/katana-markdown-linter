# MCP Distribution Plan

This document outlines the distribution strategy for `kml-mcp` (the MCP adapter for `katana-markdown-linter`) and evaluates various package types for MCP Registry publication.

## 1. Distribution Research

### 1.1 Package Type Comparison

| Package Type | Status | Pros | Cons |
| --- | --- | --- | --- |
| **MCPB** | **Primary (Desktop/CLI)** | Matches local stdio binary usage. Low overhead. | Requires manual SHA256 management in metadata. |
| **OCI (GHCR)** | **Primary (Container)** | Standard for cloud/hosted environments. Easy versioning. | Requires workspace mount and write policy docs. |
| **crates.io** | **Reference** | Source of truth for Rust users. | Not a direct MCP Registry package type yet. |
| **npm Wrapper** | **Deferred** | Familiar to Node.js users. | High ownership cost; requires binary wrapping logic. |
| **PyPI Wrapper** | **Deferred** | Familiar to Python users. | Misleading ownership; high maintenance cost. |

### 1.2 Decision: MCPB and OCI

For `v0.13.0` and beyond, we will focus on **MCPB (via GitHub Releases)** and **OCI (via GHCR)** as the official MCP distribution channels.

- **MCPB** will target desktop/CLI users who already have `cargo` or want a pre-built binary.
- **OCI** will target users who prefer containerized MCP servers.

### 1.3 Ownership Verification

- **MCPB**: Verified by the repository URL and file content.
- **OCI**: Verified by the `io.modelcontextprotocol.server.name` label in the Dockerfile/image.
- **Registry Name**: `io.github.HiroyukiFuruno/kml` (following the `io.github.owner/name` convention for GitHub-based auth).

## 2. Registry Metadata Plan

### 2.1 `server.json` Draft

A draft of `server.json` is maintained in `docs/mcp-server.md`. It includes:
- Command: `kml-mcp`
- Arguments: `["--workspace-root", "."]`
- Required features: `mcp`
- Security links: Workspace access policy documentation.

### 2.2 Ownership and Credentials

- Authentication will use **GitHub-based authentication** for the MCP Registry.
- The `mcp-publisher` tool will be used in CI to automate metadata updates.

## 3. Public Readiness Gate

Before publishing to the official MCP Registry or Hubs:
- `v0.12.8` must achieve a stable score of 90+.
- `v0.12.9` public confidence gate must pass.
- `make mcp-stdio-smoke` must be part of the release workflow.
- Security review of workspace access boundaries must be completed.
