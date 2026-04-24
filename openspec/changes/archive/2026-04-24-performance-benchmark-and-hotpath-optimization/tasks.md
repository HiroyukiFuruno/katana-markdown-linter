# Tasks

## DoR

- [x] active OpenSpec change がこの performance change のみであることを確認する
- [x] benchmark は初期段階では report-first とし、flaky な CI hard gate にしないことを確認する
- [x] 最初の optimization 対象を CLI fix mode の duplicate lint evaluation に限定する
- [x] public behavior を変えないことを design と spec で確認する

## Implementation

- [x] performance benchmark runner を追加する
- [x] synthetic corpus generation を benchmark runner に追加する
- [x] `make bench` target を追加する
- [x] `make perf-check` target を追加する
- [x] `tests/fixtures/perf-baseline.json` を追加する
- [x] `docs/performance.md` を追加する
- [x] diagnostics reuse 用の fix helper を追加する
- [x] CLI fix mode で pre-fix diagnostics を再利用する
- [x] behavior preservation test を追加する
- [x] performance report/baseline comparison を検証する

## DoD

- [x] `make bench` が成功し `target/perf-report.json` を生成する
- [x] `make perf-check` が成功し required case を検証する
- [x] `make check` が成功する
- [x] `fix(content, options)` と diagnostics reuse helper の結果が一致する
- [x] CLI fix mode の exit code と remaining diagnostics behavior が既存 test で維持される
- [x] `openspec status --change performance-benchmark-and-hotpath-optimization --json` で apply-ready である
