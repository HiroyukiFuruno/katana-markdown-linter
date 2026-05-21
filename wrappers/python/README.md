# katana-markdown-linter Python wrapper

`katana-markdown-linter` is a thin Python launcher over GitHub Release binary
archives. The package does not contain independent lint or MCP server logic. On
first use, it downloads the matching `kml`, `kml-mcp`, or `kml-mcp-remote`
binary archive, verifies the neighboring SHA-256 checksum, installs the binary
into the wrapper cache, and delegates to that binary.

## Install

~~~bash
pipx install katana-markdown-linter
kml --version
~~~

Use `uvx` for one-off runs:

~~~bash
uvx --from katana-markdown-linter==0.19.3 kml --version
uvx --from katana-markdown-linter==0.19.3 kml check README.md
~~~

## Basic Usage

~~~bash
# Run linting
kml check README.md

# Show help
kml help
kml --help
kml -h
kml check --help

# Localized help
kml --locale ja help

# Show version aliases
kml version
kml --version
kml -v

# Fix and Format
kml fix README.md
kml fmt
~~~

## MCP server entrypoints

Use `kml-mcp` for local stdio MCP clients:

~~~bash
uvx --from katana-markdown-linter==0.19.3 kml-mcp --workspace-root /absolute/path/to/workspace
~~~

Use `kml-mcp-remote` only for self-hosted Streamable HTTP:

~~~bash
KML_MCP_REMOTE_TOKEN=change-me \
  uvx --from katana-markdown-linter==0.19.3 kml-mcp-remote
~~~

## Supported Platforms

The Python launcher uses the same binary archives as the GitHub Release
channel. It currently supports:

- macOS arm64: `aarch64-apple-darwin`
- macOS x64: `x86_64-apple-darwin`
- Linux x64: `x86_64-unknown-linux-gnu`
- Windows x64: `x86_64-pc-windows-msvc`

Unsupported platforms fail before download with an explicit platform error.

## Wrapper Contract

- The package version selects the GitHub Release tag.
- The launcher downloads the matching `kml-vX.Y.Z-<target>`,
  `kml-mcp-vX.Y.Z-<target>`, or `kml-mcp-remote-vX.Y.Z-<target>` archive.
- The launcher downloads the matching `.sha256` file and verifies the archive
  before extraction.
- The installed binary is cached by version, platform, and executable role.
- MCP launchers do not write wrapper logs to stdout, preserving JSON-RPC stdio.

For full CLI usage, rule coverage, and other install channels, see the
[repository README](https://github.com/HiroyukiFuruno/katana-markdown-linter).
Report package issues through
[GitHub Issues](https://github.com/HiroyukiFuruno/katana-markdown-linter/issues).
