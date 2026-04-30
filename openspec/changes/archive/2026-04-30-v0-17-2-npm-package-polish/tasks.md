# Tasks

## Release Execution Notes

- 2026-04-30: user が npm UI で trusted publisher を `HiroyukiFuruno/katana-markdown-linter` / `release.yml` として登録済み。
- 2026-04-30: `npm view katana-markdown-linter versions --json` は `0.17.0` のみを返し、`0.17.1` は npm 未公開。
- 2026-04-30: `v0.17.1` は GitHub Release / crates.io / PyPI / Homebrew が完了し、npm は README / metadata polish 後の `v0.17.2` へ引き継ぐ。
- 2026-04-30: `v0.17.2` は npm package page / npm publish closeout に限定し、schema / editor extension work は含めない。
- 2026-04-30: `npm pack --dry-run --json` で tarball に `README.md`、`package.json`、`bin/kml.js`、`lib/installer.js` が含まれることを確認済み。

## Definition of Ready

- [x] 0.1 npm trusted publisher が `HiroyukiFuruno/katana-markdown-linter` / `release.yml` で登録済みであることを確認する
- [x] 0.2 npm `katana-markdown-linter@0.17.1` が未公開であること、または `v0.17.2` に進める理由を記録する
- [x] 0.3 `v0.17.1` の GitHub Release / crates.io / PyPI / Homebrew 完了状態と npm 未完了状態を release ledger から引き継ぐ
- [x] 0.4 `v0.17.2` は npm package page / npm publish closeout に限定し、schema / editor extension work を混ぜないことを確認する

## 1. npm Package README and Metadata

- [x] 1.1 `wrappers/npm/README.md` を追加し、install / `npx` / `kml check` の最小例を書く
- [x] 1.2 README に thin wrapper contract、GitHub Release binary archive 取得、checksum 検証、supported platforms を明記する
- [x] 1.3 `wrappers/npm/package.json` に `keywords`、`homepage`、`bugs` を追加する
- [x] 1.4 `dependencies` は不要な runtime dependency を追加せず、0 件を維持する理由を tasks または docs に記録する
- [x] 1.5 `npm pack --dry-run --json` で `README.md` が tarball に含まれることを確認する

## 2. Release Gate Updates

- [x] 2.1 npm package tarball の file list を検証する script または AST lint を追加する
- [x] 2.2 npm metadata の必須項目を検証する script または AST lint を追加する
- [x] 2.3 release workflow の npm publish 前に tarball / metadata check が実行されることを確認する
- [x] 2.4 wrapper smoke test で fresh install directory を使い、cache 済み binary による誤判定を避ける

## 3. Version and Documentation

- [x] 3.1 Cargo / npm / PyPI / MCP metadata を `0.17.2` に更新する
- [x] 3.2 `CHANGELOG.md` に npm package page polish と npm publish closeout を追加する
- [x] 3.3 `docs/distribution.md` と `docs/release-runbook.md` に npm README / metadata check を追加する
- [x] 3.4 `openspec/changes/active-roadmap.md` で `v0.17.2` を `v0.18.0` より前の npm closeout patch として扱う
- [x] 3.5 `v0-17-1-distribution-closeout/tasks.md` の npm 未完了状態を `v0.17.2` へ引き継いだことを記録する

## 4. Verification

- [x] 4.1 `make fmt-check`
- [x] 4.2 `make lint`
- [x] 4.3 `make ast-lint`
- [x] 4.4 `cargo test --workspace --locked`
- [x] 4.5 `make dogfood`
- [x] 4.6 `npm pack --dry-run --json` in `wrappers/npm`
- [x] 4.7 `make release-check VERSION=v0.17.2`
- [x] 4.8 `make release-task-ledger-check VERSION=v0.17.2`

## 5. Release Execution Tracking

OpenSpec task ledger は実装と release 前 gate までを対象にする。PR 作成、merge、publish、
post-release verification、branch hygiene は `impl-release` workflow の実行ログで追跡する。

`impl-release` 側で実行する release 手順:

- `release/v0.17.2` PR を作成し、CI と signed commit verification を確認する
- PR merge 後、`make release VERSION=v0.17.2` を正規手順で実行する
- npm publish job が trusted publishing で成功したことを確認する
- `make release-verify VERSION=v0.17.2` を実行する
- `npm view katana-markdown-linter@0.17.2 version` と `npx --yes katana-markdown-linter@0.17.2 --version` の結果を記録する
- npm package page に README、keywords、homepage、bugs が反映され、runtime dependency が 0 件であることを確認する
- `v0.18.0` 以降の schema / editor work に進める状態にする

## 品質評価スコア

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| npm README / metadata | 25 | 25 | `wrappers/npm/README.md` と package metadata を追加済み。 |
| release gate | 25 | 25 | `make npm-package-check` と release workflow publish 前 check を追加済み。 |
| release metadata | 20 | 20 | Cargo / npm / PyPI / MCP metadata と CHANGELOG を `0.17.2` に更新済み。 |
| documentation | 15 | 15 | README、distribution、release runbook、quality gates を更新済み。 |
| verification | 15 | 15 | `make release-check VERSION=v0.17.2` が成功。 |
| 合計 | 100 | 100 | release PR ready。 |
