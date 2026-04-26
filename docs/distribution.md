# Distribution Channels

## Official Channels

| Channel | Status | Verification | Policy |
| --- | --- | --- | --- |
| Cargo crate | Official | `make release-check`, install smoke test, crates.io publish verification | Primary library and CLI package |
| GitHub Action | Official from `v0.11.0` | `make action-smoke`, CI action smoke, release action smoke | CI integration over the published `kml` CLI |
| MCP Bundle (.mcpb) | Official from `v0.14.0` | `make mcp-stdio-smoke`, `make mcpb-package` | Single-click MCP server installation for desktop clients |

The GitHub Action lives at the repository root as `action.yml`, so consumers can
use the release tag directly:

~~~yaml
- uses: actions/checkout@v5
- uses: HiroyukiFuruno/katana-markdown-linter@v0.14.0
  with: { version: "0.14.0", command: check, paths: "README.md\ndocs" }
~~~

Pin both the action tag and the crate `version` input. The action installs the
published crate by default. Repository-local validation uses
`install-source: path` so pull requests verify the same action scripts without
waiting for crates.io publication.

## Deferred Channels

| Channel | Decision | Reason |
| --- | --- | --- |
| MCP Registry | Official from `v0.14.0` | Primary discovery channel for MCP servers via `server.json`. |
| pre-commit hook repository | Deferred | A dedicated hook repository adds release ownership. Local hooks can call `kml` or the GitHub Action can protect CI first. |
| Homebrew | Deferred | A tap needs stable binary archive naming or a crate-install formula policy. Current release artifacts are crate package and checksum only. |
| standalone binary artifacts | Deferred | Binary archives need platform matrix ownership and checksum verification before becoming official. |
| npm wrapper | Deferred | A Node wrapper would add another release surface before binary artifact naming is stable. |
| pip/uv wrapper | Deferred | A Python wrapper has the same binary artifact dependency as npm. |
| config schema publication | Deferred | Schema output needs versioned config metadata and editor validation tests before it can be treated as stable. |
| editor/LSP entrypoint | Deferred | `kml fmt --stdin` is editor-friendly, but a dedicated editor entrypoint should follow after distribution smoke coverage remains stable. |

## Action Inputs

| Input | Default | Meaning |
| --- | --- | --- |
| `install-source` | `crates-io` | `crates-io` installs the published package; `path` installs a local checkout. |
| `version` | empty | Crate version passed to `cargo install` for published package installs. |
| `path` | `.` | Local crate path used with `install-source: path`. |
| `command` | `check` | Supported commands are `check`, `fix`, `fmt`, `rule`, `config`, `version`, and `init-config`. |
| `paths` | `.` | Newline-separated paths passed to `check`, `fix`, or `fmt`. |
| `config` | empty | Optional markdownlint config path. |
| `locale` | `en` | CLI message locale. |
| `output` | `text` | Use `text` or `json`. |
| `extra-args` | empty | Additional `kml` arguments, one argument per line. |
