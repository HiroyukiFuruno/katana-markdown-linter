# Distribution Channels

## Official Channels

| Channel | Status | Verification | Policy |
| --- | --- | --- | --- |
| Cargo crate | Official | `make release-check`, install smoke test, crates.io publish verification | Primary library and CLI package |
| GitHub Action | Official from `v0.11.0` | `make action-smoke`, CI action smoke, release action smoke | CI integration over the published `kml` CLI |
| MCPB bundle | Official from `v0.14.0` | `make mcpb-smoke`, `make server-json-validate`, release asset checksum | Local stdio MCP package for `kml-mcp` |
| MCP Registry metadata | Official from `v0.14.0` | rendered `server.json`, MCPB checksum, registry publish verification | Discovery metadata for the MCPB bundle |
| Remote MCP server | Official from `v0.15.0` for self-hosted text-only use | `make mcp-remote-smoke`, release remote smoke | Streamable HTTP server for clients that cannot launch stdio |

The GitHub Action lives at the repository root as `action.yml`, so consumers can
use the release tag directly:

~~~yaml
- uses: actions/checkout@v5
- uses: HiroyukiFuruno/katana-markdown-linter@v0.12.1
  with: { version: "0.12.1", command: check, paths: "README.md\ndocs" }
~~~

Pin both the action tag and the crate `version` input. The action installs the
published crate by default. Repository-local validation uses
`install-source: path` so pull requests verify the same action scripts without
waiting for crates.io publication.

## Deferred Channels

| Channel | Decision | Reason |
| --- | --- | --- |
| pre-commit hook repository | Deferred | A dedicated hook repository adds release ownership. Local hooks can call `kml` or the GitHub Action can protect CI first. |
| Homebrew | Deferred | A tap needs stable binary archive naming or a crate-install formula policy. Current release artifacts are crate package and checksum only. |
| standalone binary artifacts | Deferred | General CLI binary archives need platform matrix ownership and checksum verification before becoming official. |
| npm wrapper | Deferred | A Node wrapper would add another release surface before binary artifact naming is stable. |
| pip/uv wrapper | Deferred | A Python wrapper has the same binary artifact dependency as npm. |
| config schema publication | Deferred | Schema output needs versioned config metadata and editor validation tests before it can be treated as stable. |
| editor/LSP entrypoint | Deferred | `kml fmt --stdin` is editor-friendly, but a dedicated editor entrypoint should follow after distribution smoke coverage remains stable. |

The first MCPB bundle is a Linux release-runner artifact for the local stdio
server. Wider platform-specific MCPB or standalone binary artifacts belong to
the binary distribution expansion track.

`kml-mcp-remote` is not a hosted service and is not described by the MCPB
Registry metadata. Operators deploy it themselves, keep bearer authentication
enabled, and expose only text-only tools in `v0.15.0`.

## Action Inputs

| Input | Default | Meaning |
| --- | --- | --- |
| `install-source` | `crates-io` | `crates-io` installs the published package; `path` installs a local checkout. |
| `version` | empty | Crate version passed to `cargo install` for published package installs. |
| `path` | `.` | Local crate path used with `install-source: path`. |
| `command` | `check` | Supported commands are `check`, `fix`, `fmt`, `rule`, `config`, `version`, and `init-config`. |
| `paths` | `.` | Newline-separated paths passed to `check`, `fix`, or `fmt`. |
| `config` | empty | Optional markdownlint config path. |
| `locale` | `en` | CLI message locale (`en`, `ja`, `zh-CN`, `zh-TW`, `ko`, `pt`, `fr`, `de`, `es`, or `it`). |
| `output` | `text` | Use `text` or `json`. |
| `extra-args` | empty | Additional `kml` arguments, one argument per line. |
