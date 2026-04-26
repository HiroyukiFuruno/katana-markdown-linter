## Design

## Measurement

Performance work は以下の順に行う。

1. current baseline を `make bench` と `make bench-cross-tools-*` で取得する
2. hot path を profiler または sampling tool で特定する
3. 1 optimization につき 1 hypothesis を立てる
4. behavior tests と benchmark を両方通す
5. docs に before/after を記録する

Profiler は環境差があるため、`cargo flamegraph`, `samply`, macOS `sample`, internal timing counters のいずれかを使う。
使った tool と command は docs に記録する。

## Comparison Policy

- `default` mode: each tool default behavior。参考値として扱う
- `common` mode: common candidate rules config。比較の主指標にする
- `fix` mode: temporary workspace copy を使い source fixture を変更しない
- missing optional tool は skip とし、report に残す

## Candidate Hot Paths

- CLI directory traversal and ignore matching
- config file discovery / parsing / validation
- rule registry construction
- repeated regex compilation
- per-line string allocation
- fix loop duplicate linting
- output serialization
- JSON report construction

## Optimization Policy

- behavior change を performance change に混ぜない
- public API compatibility を維持する
- micro-optimization より algorithmic duplicate work の除去を優先する
- benchmark baseline refresh は intentional optimization と同じ change で行う

## Non-Goals

- CI wall-clock hard gate
- `mado` / `rumdl` の完全互換
- unsafe code による最適化
