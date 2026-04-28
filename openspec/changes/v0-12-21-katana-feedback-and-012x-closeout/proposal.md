## Why

`v0.12.20` で性能計測と改善を終えた後、実運用に近い KatanA 側ドキュメントを読ませて feedback を取り込みたい。
`v0.12.21` は、その時点で issue があれば bugfix として扱い、なければ残り rule の by-design 宣言で 0.12.x を閉じる。

## What Changes

- `v0.12.20` 完了後に KatanA 側の複数ドキュメントを external corpus として確認する。
- KatanA feedback で見つかった issue は分類し、0.12.x closeout に必要なものを `v0.12.21` で修正する。
- release-blocking ではない issue は、後続版の follow-up として明示する。
- 残りの `Diagnostic only` / `Needs triage` ルールについて、safe-fix を実装しない理由を README、fixture matrix、関連 docs に反映する。
- `MD028` が `v0.12.19` で safe-fix 実装されなかった場合、`v0.12.21` の by-design 宣言対象に含める。

## Capabilities

### New Capabilities

### Modified Capabilities

- `dogfood-workflow`: KatanA 側ドキュメントを feedback sweep として扱い、finding を分類して `v0.12.21` の scope に取り込む。
- `rule-coverage`: 残り diagnostic-only rule の by-design 理由を公開 rule map と fixture に反映する。
- `release-readiness`: 0.12.x closeout から `v0.13.0` へ進む条件を明確にする。

## Impact

- KatanA checkout を使う optional dogfood / public-confidence 実行
- `README.md`
- `docs/rule-fix-feasibility.md`
- `docs/rule-coverage-dashboard.md`
- `tests/fixtures/rule-fixture-matrix.json`
- `tests/fixtures/rule-fixture-matrix.md`
- `CHANGELOG.md`
- `Cargo.toml`
- `Cargo.lock`
- `openspec/changes/active-roadmap.md`
