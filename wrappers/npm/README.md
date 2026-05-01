# katana-markdown-linter

`katana-markdown-linter` is a thin npm launcher for the `kml` Markdown linter.
The package does not contain independent lint logic. On first use, it downloads
the matching `kml` binary archive from GitHub Releases, verifies the neighboring
SHA-256 checksum, installs the binary into the wrapper cache, and then delegates
all commands to that binary, including localized CLI help.

## Install

~~~bash
npm install -g katana-markdown-linter
kml --version
~~~

Use `npx` for one-off runs:

~~~bash
npx --yes katana-markdown-linter@0.18.7 --version
npx --yes katana-markdown-linter@0.18.7 check README.md
~~~

## Basic Usage

~~~bash
kml check README.md
kml check docs --config .markdownlint.json
kml --locale ja help
kml fix README.md
~~~

## Supported Platforms

The npm launcher uses the same binary archives as the GitHub Release channel.
It currently supports:

- macOS arm64: `aarch64-apple-darwin`
- macOS x64: `x86_64-apple-darwin`
- Linux x64: `x86_64-unknown-linux-gnu`
- Windows x64: `x86_64-pc-windows-msvc`

Unsupported platforms fail before download with an explicit platform error.

## Wrapper Contract

- The package version selects the GitHub Release tag.
- The launcher downloads `kml-vX.Y.Z-<target>.tar.gz` or
  `kml-vX.Y.Z-<target>.zip`.
- The launcher downloads the matching `.sha256` file and verifies the archive
  before extraction.
- The installed binary is cached under the package-local `vendor` directory by
  default.

For full CLI usage, rule coverage, and other install channels, see the
[repository README](https://github.com/HiroyukiFuruno/katana-markdown-linter).
Report package issues through
[GitHub Issues](https://github.com/HiroyukiFuruno/katana-markdown-linter/issues).
