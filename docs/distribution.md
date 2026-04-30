# Distribution Channels

## Official Channels

| Channel | Status | Verification | Policy |
| --- | --- | --- | --- |
| Cargo crate | Official | `make release-check`, install smoke test, crates.io publish verification | Primary library and CLI package |
| Standalone binary artifacts | Official from `v0.17.1` | `make binary-smoke`, release asset checksum, `make release-verify` asset check | Rust-toolchain-free CLI installs from GitHub Releases |
| npm wrapper | Official from `v0.17.3` | npm registry version, `npx` launch smoke, package README / metadata check, `make release-verify` | Thin launcher over GitHub Release binary archives |
| PyPI wrapper | Official from `v0.17.1` | PyPI JSON version, `uvx` launch smoke, package README / metadata check, `make release-verify` | Thin launcher over GitHub Release binary archives |
| Homebrew formula | Official from `v0.17.1` | `make homebrew-formula-check`, release archive checksum, formula test block, actual tap check in `make release-verify` | Release workflow updates latest and versioned formulae in `homebrew-katana` after release assets exist |
| GitHub Action | Official from `v0.11.0` | `make action-smoke`, CI action smoke, release action smoke | CI integration over the published `kml` CLI |
| MCPB bundle | Official from `v0.14.0` | `make mcpb-smoke`, `make server-json-validate`, release asset checksum | Local stdio MCP package for `kml-mcp` |
| MCP Registry metadata | Official from `v0.14.0` | rendered `server.json`, MCPB checksum, registry publish verification | Discovery metadata for the MCPB bundle |
| Remote MCP server | Official from `v0.15.0` for self-hosted text-only use | `make mcp-remote-smoke`, release remote smoke | Streamable HTTP server for clients that cannot launch stdio |

The GitHub Action lives at the repository root as `action.yml`, so consumers can
use the release tag directly:

~~~yaml
- uses: actions/checkout@v6
- uses: HiroyukiFuruno/katana-markdown-linter@v0.17.6
  with: { version: "0.17.6", command: check, paths: "README.md\ndocs" }
~~~

Pin both the action tag and the crate `version` input. The action installs the
published crate by default. Repository-local validation uses
`install-source: path` so pull requests verify the same action scripts without
waiting for crates.io publication.

## Deferred Channels

| Channel | Decision | Reason |
| --- | --- | --- |
| pre-commit hook repository | Deferred | A dedicated hook repository adds release ownership. Local hooks can call `kml` or the GitHub Action can protect CI first. |
| config schema publication | Deferred | Schema output needs versioned config metadata and editor validation tests before it can be treated as stable. |
| editor/LSP entrypoint | Deferred | `kml fmt --stdin` is editor-friendly, but a dedicated editor entrypoint should follow after distribution smoke coverage remains stable. |

Standalone release archives use stable names such as
`kml-v0.17.6-aarch64-apple-darwin.tar.gz` and always ship a neighboring
`.sha256` file. The Homebrew formula is generated from the same release assets
and is published to `HiroyukiFuruno/homebrew-katana` by the release workflow
with `HOMEBREW_KATANA_GIT_TOKEN`. Each release updates the latest `kml`
formula and adds a versioned `kml@X.Y.Z` formula.

The npm and PyPI packages do not contain independent lint logic. They download
the matching GitHub Release archive for the package version, verify the archive
checksum, and launch the bundled `kml` binary. Both wrapper packages ship
registry README and metadata that are verified by `make npm-package-check` and
`make pypi-package-check` before publication.

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
