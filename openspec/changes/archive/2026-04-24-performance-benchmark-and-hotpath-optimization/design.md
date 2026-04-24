# Design

## Goals

- ms 単位の性能改善を議論できる benchmark report を作る
- CI 環境差で flaky にしないため、初期は report-first にする
- hot path を推測ではなく baseline 付きで改善する
- public API と CLI の責務を崩さず、既存 behavior を維持する

## Non-Goals

- 初期 change で厳密な CI performance failure を導入しない
- OS / CPU ごとの絶対時間比較を hard gate にしない
- rule architecture 全体の大規模 rewrite は行わない

## Benchmark Scope

初期 benchmark は synthetic corpus を使う。

| Case | Purpose |
| --- | --- |
| `api_lint_large_document` | single large Markdown content の rule evaluation cost を測る |
| `api_fix_large_document` | safe fix path の cost を測る |
| `api_lint_many_small_documents` | small file が多い workspace 相当の repeated API overhead を測る |
| `api_rule_catalog` | rule metadata/catalog construction cost を測る |

Synthetic corpus は runner 内で生成する。
外部 fixture の状態や filesystem cache に結果を引きずられないよう、初期 report は API benchmark を中心にする。

## Report Schema

`make bench` は `target/perf-report.json` を生成する。

各 entry は以下を含む。

- case name
- iterations
- total milliseconds
- average milliseconds
- throughput values when present

`make perf-check` は `tests/fixtures/perf-baseline.json` と current report を比較する。
初期は threshold を広めに取り、差分を可視化する。
hard fail は schema error や missing case に限定する。

## First Hot Path Optimization

CLI fix mode currently does:

1. lint original content
2. call `fix(content, options)`
3. `fix()` lints original content again
4. lint fixed content

The optimized path keeps `fix(content, options)` for public convenience but adds an internal/public helper that applies fixes from already computed diagnostics.
CLI fix mode then reuses the first diagnostics and avoids the second lint.

This keeps external behavior unchanged:

- same fixed content
- same applied fix count
- same remaining diagnostics after fix
- same exit codes

## Verification

- `make bench`
- `make perf-check`
- existing `make check`
- unit test that `fix_with_results` matches `fix`
