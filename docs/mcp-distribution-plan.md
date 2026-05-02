# MCP Distribution Plan

This document records the `v0.13.0` distribution decision for `kml-mcp`, the
MCP adapter for `katana-markdown-linter`.

## Decision

`v0.13.0` fixed the package strategy and release gates. `v0.14.0` implements
the first public MCP package and Registry metadata for `kml-mcp`.

The selected distribution channels are:

| Channel | Role | Reason |
| --- | --- | --- |
| MCPB from GitHub Releases | Primary local package | It matches local stdio server use and can be attached to versioned releases. |
| OCI image on GHCR | Deferred container package | It needs image ownership, labels, and workspace mount policy before publication. |
| crates.io binary | Rust install source | It remains the Rust package source but is not an MCP Registry package type. |
| npm wrapper | Deferred fallback | It would add wrapper maintenance without improving the Rust binary contract. |
| PyPI wrapper | Deferred fallback | It would imply Python ownership that the project does not have. |
| NuGet package | Not planned | The project has no .NET distribution surface. |

The MCP Registry is metadata, not artifact hosting. Registry metadata will point
to public packages or release artifacts after those artifacts exist.

## Registry Package Notes

MCPB publication uses a GitHub Release artifact whose URL contains `mcp` and a
`fileSha256` value in the rendered `server.json`. MCP clients use that hash when
installing the bundle.

OCI publication needs an image on a supported registry. GHCR remains the
selected future registry for this repository, but it is deferred until an image
build, label policy, and workspace mount contract are implemented.

npm, PyPI, and NuGet are supported registry package types, but they are not
selected for this project because `kml-mcp` is a Rust binary and those wrappers
would become extra product surfaces.

## Metadata Plan

The source `server.json` lives at the repository root. It records:

- server name: `io.github.HiroyukiFuruno/kml`
- binary: `kml-mcp`
- transport: stdio
- selected package: MCPB
- workspace safety documentation: `docs/mcp-server.md`
- publish timing: release workflow after the MCPB artifact exists

The release workflow renders `target/mcpb/server.json` with the actual GitHub
Release artifact URL and SHA-256 checksum before publication.

## Public Readiness Gate

Registry or hub publication remains blocked until:

- the MCPB artifact is built by the release workflow
- `just mcp-stdio-smoke` passes against the installable binary
- workspace root enforcement and explicit file apply behavior are documented
- the rendered metadata passes repository validation
- `v0.14.0` explicitly performs the publish step

Remote MCP transport is not part of this plan. If API-hosted LLM access becomes
necessary, it belongs to `v0.15.0`.

## References

- [MCP Registry overview](https://modelcontextprotocol.io/registry/about)
- [MCP Registry package types](https://modelcontextprotocol.io/registry/package-types)
- [MCP Registry authentication](https://modelcontextprotocol.io/registry/authentication)
