# Tasks

## DoR

- [x] active OpenSpec change がこの change のみであることを確認する
- [x] 現段階では timing regression を CI hard fail にしないことを確認する
- [x] public API と CLI behavior を変更しないことを確認する
- [x] benchmark schema update と baseline refresh を同じ change に含めることを確認する

## Implementation

- [x] benchmark runner に warmup と repeated sample collection を追加する
- [x] benchmark report schema に samples、mean、median、min、max、stddev を追加する
- [x] clean large document lint benchmark case を追加する
- [x] CLI directory check benchmark case を追加する
- [x] config validation benchmark case を追加する
- [x] `perf-check.py` に schema/statistic validation を追加する
- [x] `perf-check.py` の comparison summary を median 中心に変更する
- [x] `tests/fixtures/perf-baseline.json` を新 schema で更新する
- [x] `docs/performance.md` を新 schema と測定方針に更新する
- [x] OpenSpec main spec に delta を同期する

## DoD

- [x] `make bench` が成功し新 schema の `target/perf-report.json` を生成する
- [x] `make perf-check` が成功し required case と statistic fields を検証する
- [x] `make check` が成功する
- [x] `openspec status --change performance-measurement-hardening --json` で apply-ready である
- [x] active OpenSpec change が archive 可能な状態である
