## Why

`kml` を高性能 Markdown linter として育てるには、個別最適化より先に継続測定できる基盤が必要である。
現状は `make check` や upstream golden gate はあるが、large document、many files、fix path の実行時間を ms 単位で比較する基準がない。

加えて、CLI fix mode は fix 前 diagnostics を取得した後に `fix()` 内で同じ content を再 lint しており、明確な重複評価がある。
最初の performance change では、この重複を潰しつつ、今後の高速化を測れる benchmark/report を追加する。

## What Changes

- deterministic な benchmark corpus generator と benchmark runner を追加する
- `make bench` で release build の performance report を生成できるようにする
- `make perf-check` で baseline と比較し、当面は report-first の regression visibility を提供する
- `docs/performance.md` に測定対象、結果の読み方、baseline 更新方針を記載する
- CLI fix mode が既に取得した diagnostics を再利用し、同じ content への重複 lint を避ける
- OpenSpec に performance measurement / regression visibility の contract を追加する

## Capabilities

### New Capabilities

- `performance-benchmarking`: benchmark scope、report schema、baseline comparison、初期 hot path optimization を定義する

## Impact

- `Makefile`
- `examples/`
- `src/lib.rs`
- `src/cli.rs`
- `docs/`
- `tests/fixtures/`
- `openspec/specs/`
