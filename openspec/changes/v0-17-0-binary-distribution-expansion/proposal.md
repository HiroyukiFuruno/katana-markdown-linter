# Binary Distribution Expansion

## Target Version

`v0.17.0`

## Why

`v0.16.x` までで Cargo、GitHub Action、MCPB、Remote MCP、editor 向けの基本導線は揃った。
次は Rust toolchain を直接使わない利用者が、OS ごとの binary と Homebrew から `kml` を導入できる状態にする。

現在の `docs/distribution.md` では Homebrew、standalone binary artifacts、npm wrapper、pip/uv wrapper が deferred のままである。
`v0.17.0` では binary artifact 名、checksum、smoke test、Homebrew formula 更新の責務を固定し、wrapper 系は binary artifact contract に従う薄い配布導線として扱う。

## What Changes

- GitHub Release に `kml` の OS / architecture 別 standalone binary artifact と checksum を追加する
- release workflow と local `make release-check` に binary build / archive / install smoke を追加する
- Homebrew formula 更新に必要な artifact URL、checksum、version 検証を release flow に組み込む
- `docs/distribution.md` と README の install section を、Cargo 以外の公式導線まで広げる
- npm / pip wrapper は Rust 実装を持たず、公式 binary artifact を取得して `kml` を実行する thin wrapper として要件を定義する
- wrapper publish を同時に行う場合でも、primary distribution は GitHub Release binary と Homebrew に置く

## Capabilities

### New Capabilities

- `binary-distribution`: `kml` standalone binary artifact、checksum、archive naming、Homebrew、npm / pip wrapper の配布契約を扱う

### Modified Capabilities

- `release-cicd`: release workflow が `.crate` / MCPB に加えて `kml` binary artifact を作り、smoke test と checksum を通したうえで GitHub Release に添付する
- `release-readiness`: release 前確認が Cargo install だけでなく、standalone binary と Homebrew formula の導入確認を含む

## Impact

- `.github/workflows/release.yml`
- `Makefile`
- `scripts/release/**`
- binary artifact 用の manifest / packaging script
- Homebrew formula 更新または tap 連携 script
- npm / pip wrapper を置く場合の package metadata と smoke test
- `README.md`
- `docs/distribution.md`
- `CHANGELOG.md`
- OpenSpec roadmap と対象 change archive

## Non-Goals

- Markdown lint engine の rule / fix 追加
- KatanA 専用 adapter の実装
- Homebrew tap repository 側の未承認操作を、検証なしに自動 publish すること
- npm / pip wrapper に独自の Markdown lint 実装を持たせること
- package manager ごとに異なる CLI contract を作ること
