# MCP Distribution Plan

This document records the `v0.13.0` distribution decision for `kml-mcp`, the
MCP adapter for `katana-markdown-linter`.

## Decision

`v0.13.0` does not publish `kml-mcp` to the MCP Registry or third-party MCP
hubs. It fixes the package strategy and release gates so the publication work
can happen in `v0.14.0`.

The selected distribution channels are:

| Channel | Role | Reason |
| --- | --- | --- |
| MCPB from GitHub Releases | Primary local package | It matches local stdio server use and can be attached to versioned releases. |
| OCI image on GHCR | Primary container package | It gives hosted and container-first users a standard pull target. |
| crates.io binary | Rust install source | It remains the Rust package source but is not an MCP Registry package type. |
| npm wrapper | Deferred fallback | It would add wrapper maintenance without improving the Rust binary contract. |
| PyPI wrapper | Deferred fallback | It would imply Python ownership that the project does not have. |
| NuGet package | Not planned | The project has no .NET distribution surface. |

The MCP Registry is metadata, not artifact hosting. Registry metadata will point
to public packages or release artifacts after those artifacts exist.

## Registry Package Notes

MCPB publication needs a GitHub Release artifact whose URL contains `mcp` and a
`fileSha256` value in `server.json`. MCP clients use that hash when installing
the bundle.

OCI publication needs an image on a supported registry. GHCR is the selected
registry for this repository. The image must carry the
`io.modelcontextprotocol.server.name` annotation with the same value as the
server name in `server.json`.

npm, PyPI, and NuGet are supported registry package types, but they are not
selected for this project because `kml-mcp` is a Rust binary and those wrappers
would become extra product surfaces.

## Metadata Plan

The draft `server.json` lives in `docs/mcp-server.md`. The draft records:

- server name: `io.github.HiroyukiFuruno/kml`
- binary: `kml-mcp`
- transport: stdio
- selected packages: MCPB and OCI
- workspace safety documentation: `docs/mcp-server.md`
- publish timing: deferred to `v0.14.0`

The release implementation must replace placeholder hashes and artifact
references before publication.

## Public Readiness Gate

Registry or hub publication remains blocked until:

- the MCPB artifact is built by the release workflow
- the OCI image is built and labeled with the MCP server name
- `make mcp-stdio-smoke` passes against the installable binary
- workspace root enforcement and explicit file apply behavior are documented
- the draft metadata passes `mcp-publisher` validation
- `v0.14.0` explicitly performs the publish step

Remote MCP transport is not part of this plan. If API-hosted LLM access becomes
necessary, it belongs to `v0.15.0`.

## References

- [MCP Registry overview](https://modelcontextprotocol.io/registry/about)
- [MCP Registry package types](https://modelcontextprotocol.io/registry/package-types)
- [MCP Registry authentication](https://modelcontextprotocol.io/registry/authentication)
