## Context

このリポジトリには既に性能計測の入口がある。
`make bench` は `target/perf-report.json` を生成し、`make perf-check` は committed baseline と比較する。
`make perf-check-strict` は median 比率が閾値を超えた場合に失敗する。

`v0.12.20` では新しい機能を足すより、`v0.12.19` で増えた rule/fix の影響を測ることを優先する。

## Goals / Non-Goals

**Goals:**

- `v0.12.19` 後の性能状態を記録する。
- 説明不能な退行があれば、原因を特定して修正する。
- 改善は正しさを変えない内部最適化に限定する。
- `v0.12.21` の KatanA feedback sweep に進める状態を作る。

**Non-Goals:**

- 新しい rule や safe-fix を追加しない。
- benchmark 数値のために lint semantics を弱めない。
- required CI を外部 tool (`mado`、`rumdl`、`hyperfine`) に依存させない。

## Decisions

### D-1. まず既存 benchmark で測る

測定の主入口は次に固定する。

- `make bench`
- `make perf-check`
- `make perf-check-strict`
- `make public-confidence`

cross-tool benchmark は任意 evidence とし、optional tool がない場合は skipped として扱う。

### D-2. 改善対象は測定で選ぶ

事前に hot path を決め打ちしない。
候補は次のように分類する。

- `api_fix_large_document` または `api_fix_parser_heavy_document` が悪化した場合: fix convergence と range application を確認する。
- `context_*` が悪化した場合: `DocumentContext` の lazy index と重複構築を確認する。
- `cli_*_many_small_files` が悪化した場合: CLI traversal、config validation、file IO の分離を確認する。

### D-3. baseline refresh は根拠がある場合だけ行う

`tests/fixtures/perf-baseline.json` は、改善が確認できた場合、または benchmark case の意図的な変更がある場合だけ更新する。
単なる環境ゆらぎでは更新しない。

## Risks / Trade-offs

- [Risk] ローカル実行の時計時間だけで過剰反応する。
  - Mitigation: median 比較と strict threshold を使い、数値だけでなく原因を記録する。
- [Risk] 速度改善のために rule behavior を変えてしまう。
  - Mitigation: `make test`、`make public-confidence`、`make dogfood` を性能変更後に通す。
- [Risk] cross-tool benchmark が環境依存になる。
  - Mitigation: required gate にせず、任意 evidence として扱う。
