# v0.17.4 wrapper cache and verification recovery tasks

## Context

- 2026-04-30: `v0.17.3` の公開後検証で、PyPI wrapper が既存の `~/.cache/kml-wrapper/bin/kml` を再利用し、`uvx --from katana-markdown-linter==0.17.3 kml --version` が `0.17.0` を返した。
- 2026-04-30: `make release-verify VERSION=v0.17.3` は、手元に release tag がない状態で失敗し、さらに release asset 名確認の `grep -q` / here-string 経路で停止した。
- 2026-04-30: 公開済み registry artifact は差し替えできないため、修正版は `v0.17.4` として扱う。

## Tasks

- [x] 1.1 npm wrapper の binary cache path を version / target 別に分離する
- [x] 1.2 PyPI wrapper の binary cache path を version / target 別に分離する
- [x] 1.3 wrapper smoke に unversioned stale cache を再利用しない回帰検証を追加する
- [x] 2.1 release tag verification が手元にない tag を origin から取得できるようにする
- [x] 2.2 release asset 名確認を pipe / `grep -q` に依存しない判定へ置き換える
- [x] 3.1 Cargo / npm / PyPI / MCP metadata を `0.17.4` に更新する
- [x] 3.2 README / distribution docs / wrapper README / CHANGELOG を `0.17.4` に更新する
- [x] 4.1 `make wrapper-smoke VERSION=v0.17.4`
- [x] 4.2 `make pypi-package-check`
- [x] 4.3 `make npm-package-check`
- [x] 4.4 `make ast-lint`
- [x] 4.5 `make dogfood`
- [x] 4.6 `scripts/openspec validate release-readiness --strict`
- [x] 4.7 `scripts/openspec validate release-cicd --strict`
- [x] 4.8 `scripts/openspec validate binary-distribution --strict`
- [x] 4.9 `make release-task-ledger-check VERSION=v0.17.4`
- [x] 4.10 `make release-check VERSION=v0.17.4`
- [x] 5.1 `release/v0.17.4` PR を作成し、CI と signed commit verification を確認する
- [x] 5.2 merge 後の Release workflow が GitHub Release / crates.io / npm / PyPI を同一 run で公開することを確認する
- [x] 5.3 `make release-verify VERSION=v0.17.4` を stale cache が残った状態で実行し、`npm_wrapper_version=0.17.4` と `pypi_wrapper_version=0.17.4` を確認する

## Quality Score

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| wrapper cache | 25 | 25 | npm / PyPI wrapper cache を version / target 別に分離し、stale cache smoke を追加済み。 |
| verification flow | 25 | 25 | tag auto-fetch と pipe-free asset check により、事後検証の停止箇所を除去済み。 |
| release metadata | 20 | 20 | Cargo / npm / PyPI / MCP metadata と公開 docs を `0.17.4` に更新済み。 |
| release automation | 15 | 15 | release PR merge 経路で wrappers を含む同一 run publish を維持。 |
| evidence | 15 | 15 | local release gates、PR CI、post-release registry smoke を完了。 |
| 合計 | 100 | 100 | v0.17.4 release readiness 達成。 |
