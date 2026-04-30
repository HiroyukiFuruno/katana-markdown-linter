# Tasks

## Release Execution Notes

- 2026-04-30: `v0.17.2` は tag push trigger の Release run `25166227395` により GitHub Release / crates.io だけ先行公開された。
- 2026-04-30: npm `katana-markdown-linter@0.17.2` と PyPI `katana-markdown-linter==0.17.2` は未公開であることを確認した。
- 2026-04-30: 公開済み crates.io version は差し替えできないため、整合版は `v0.17.3` とする。
- 2026-04-30: `v0.17.3` は npm README / metadata closeout に加えて、PyPI project page polish と release workflow の partial publish 防止を含める。

## 1. PyPI Package Page

- [x] 1.1 `wrappers/python/README.md` に install / `uvx` / `kml check` の例を書く
- [x] 1.2 README に thin wrapper contract、GitHub Release binary archive 取得、checksum 検証、supported platforms を明記する
- [x] 1.3 `wrappers/python/pyproject.toml` に `keywords`、classifiers、project URLs を追加する
- [x] 1.4 runtime dependency は 0 件を維持する

## 2. Release Gate Updates

- [x] 2.1 `make pypi-package-check` を追加し、README / metadata / sdist / wheel / wheel metadata を検証する
- [x] 2.2 release-preflight と release workflow に `make pypi-package-check` を追加する
- [x] 2.3 `release/vX.Y.Z` PR merge で crates.io / npm / PyPI が同じ Release run から publish されるよう workflow を修正する
- [x] 2.4 tag push trigger を release publication path から外し、manual dispatch との二重実行を避ける

## 3. Version and Documentation

- [x] 3.1 Cargo / npm / PyPI / MCP metadata を `0.17.3` に更新する
- [x] 3.2 `CHANGELOG.md` に PyPI page polish と release flow recovery を追加する
- [x] 3.3 `docs/distribution.md`、`docs/release-runbook.md`、`docs/quality-gates.md` に PyPI package check と release PR 自動公開 flow を追加する
- [x] 3.4 `openspec/changes/active-roadmap.md` で `v0.17.3` を `v0.18.0` より前の整合版 release として扱う

## 4. Verification

- [x] 4.1 `make pypi-package-check`
- [x] 4.2 `make npm-package-check`
- [x] 4.3 `make ast-lint`
- [x] 4.4 `make dogfood`
- [x] 4.5 `scripts/openspec validate release-cicd --strict`
- [x] 4.6 `scripts/openspec validate release-readiness --strict`
- [x] 4.7 `scripts/openspec validate binary-distribution --strict`
- [x] 4.8 `make release-check VERSION=v0.17.3`
- [x] 4.9 `make release-task-ledger-check VERSION=v0.17.3`

## 5. Release Execution Tracking

OpenSpec task ledger は実装と release 前 gate までを対象にする。PR 作成、merge、publish、
post-release verification、branch hygiene は `impl-release` workflow の実行ログで追跡する。

`impl-release` 側で実行する release 手順:

- `release/v0.17.3` PR を作成し、CI と signed commit verification を確認する
- PR merge 後の Release workflow が GitHub Release / crates.io / npm / PyPI を同一 run で publish することを確認する
- `make release-verify VERSION=v0.17.3` を実行する
- `npm view katana-markdown-linter@0.17.3 version` と `npx --yes katana-markdown-linter@0.17.3 --version` の結果を記録する
- PyPI project page に README、keywords、project URLs、runtime dependency 0 件が反映されることを確認する
- `v0.18.0` 以降の schema / editor work に進める状態にする

## 品質評価スコア

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| PyPI README / metadata | 25 | 25 | `wrappers/python/README.md` と package metadata を追加済み。 |
| release flow | 25 | 25 | tag push trigger を外し、release PR merge で crates.io / npm / PyPI を同一 run に統一済み。 |
| release metadata | 20 | 20 | Cargo / npm / PyPI / MCP metadata と CHANGELOG を `0.17.3` に更新済み。 |
| documentation | 15 | 15 | README、distribution、release runbook、quality gates、OpenSpec specs を更新済み。 |
| verification | 15 | 15 | `make release-check VERSION=v0.17.3` が成功。 |
| 合計 | 100 | 100 | release PR ready。 |
