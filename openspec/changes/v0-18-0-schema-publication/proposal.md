# Config Schema Publication

## Target Version

`v0.18.0`

## Why

`kml config schema` と `schema/markdownlint.schema.json` は存在するが、
公開 URL、versioned schema、回帰検証、editor docs が release gate としてまだ固定されていない。

VS Code / Zed extension を作る前に、editor が依存できる設定スキーマ（schema）の
公開契約を固める。

## What Changes

- committed schema file と `kml config schema` 出力が一致することを検証する
- stable schema URL と versioned schema artifact の扱いを固定する
- schema regression test を追加し、rule metadata 変更時の意図しない schema 差分を検出する
- `.markdownlint.json` / `.markdownlint.jsonc` の editor validation docs を更新する
- release workflow と local release gate に schema publication check を追加する
- schema の compatibility policy を docs に明記する

## Capabilities

### New Capabilities

- `config-schema-publication`: published config schema、versioned artifact、compatibility check を扱う

### Modified Capabilities

- `cli`: `kml config schema` の published schema contract を固定する
- `release-readiness`: release gate に schema publication evidence を追加する

## Impact

- `schema/markdownlint.schema.json`
- `src/config/schema.rs`
- `src/cli/workflow/config_cmd.rs`
- `tests/cli_core_contract.rs`
- `tests/ast_linter.rs`
- `.github/workflows/release.yml`
- `.github/workflows/release-preflight.yml`
- `Makefile`
- `scripts/release/**`
- `README.md`
- `docs/editor-integration.md`
- `docs/distribution.md`
- `docs/quality-gates.md`
- `docs/release-runbook.md`
