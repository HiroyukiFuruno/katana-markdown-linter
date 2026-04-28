## Why

`v0.12.19` で safe-fix 対象が増えると、lint / fix の hot path に新しい負荷が乗る可能性がある。
`v0.12.20` は機能追加を止め、性能計測（performance）と必要最小限の改善だけに集中する。

## What Changes

- `v0.12.19` 適用後の性能を既存 benchmark で測定する。
- `make bench`、`make perf-check`、`make perf-check-strict` を主要 evidence とする。
- 必要に応じて cross-tool benchmark と public confidence timing も記録する。
- 説明できない性能退行がある場合だけ、正しさを変えない hot path 改善を行う。
- baseline refresh は、改善または意図した測定形状変更がある場合だけ行う。

## Capabilities

### New Capabilities

### Modified Capabilities

- `performance-benchmarking`: `v0.12.19` 後の性能測定と改善判断を release evidence として固定する。
- `release-readiness`: 性能改善が正しさを弱めていないことを `v0.12.20` の release gate に追加する。

## Impact

- `examples/perf_benchmark.rs`
- `scripts/ci/perf-check.py`
- `tests/fixtures/perf-baseline.json`
- `docs/performance.md`
- `CHANGELOG.md`
- `Cargo.toml`
- `Cargo.lock`
- `openspec/changes/active-roadmap.md`
