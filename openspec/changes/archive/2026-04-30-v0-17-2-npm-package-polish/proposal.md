# npm Package Polish

## Target Version

`v0.17.2`

## Why

`v0.17.1` では GitHub Release / crates.io / PyPI / Homebrew tap は整ったが、
npm package page は README と keywords が空のままで、初見の user が導入方法や
thin wrapper の性質を判断しにくい。

`v0.17.2` では npm wrapper の公開をやり直す前に、npm registry 上で見える
package metadata と README を整え、公開後検証まで含めて npm channel を閉じる。

## What Changes

- `wrappers/npm/README.md` を追加し、npm package page に install / npx / thin wrapper contract / supported platforms を表示する
- npm `package.json` に `keywords`、`homepage`、`bugs`、`readme` 相当の metadata を追加する
- npm package tarball に README が含まれることを `npm pack --dry-run --json` で検証する
- npm wrapper publish を trusted publishing で再実行し、`npm view` と `npx` で `0.17.2` を確認する
- v0.17.1 で残った npm blocker を v0.17.2 の release ledger に引き継ぐ

## Capabilities

### New Capabilities

なし。

### Modified Capabilities

- `binary-distribution`: npm wrapper package が registry page で導入方法と thin wrapper 境界を説明できることを追加する
- `release-cicd`: npm publish 前に package tarball の README / metadata を検証する条件を追加する
- `release-readiness`: npm registry state と wrapper launch を v0.17.2 の完了条件として扱う

## Impact

- `wrappers/npm/package.json`
- `wrappers/npm/README.md`
- `wrappers/npm/**`
- `.github/workflows/release.yml`
- `Makefile`
- `scripts/release/**`
- `tests/ast_linter.rs`
- `docs/release-runbook.md`
- `docs/distribution.md`
- `docs/quality-gates.md`
- `CHANGELOG.md`
- `openspec/changes/v0-17-1-distribution-closeout/tasks.md`
- `openspec/changes/active-roadmap.md`
