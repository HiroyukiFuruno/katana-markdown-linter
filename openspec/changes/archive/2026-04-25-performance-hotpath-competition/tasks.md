## Definition of Ready

- [x] `performance-measurement-hardening` と `cross-tool-cli-benchmark` が archive 済みであること
- [x] `make bench` が current baseline を生成できること
- [x] `make bench-cross-tools-common` が optional tools missing 時に skip できること
- [x] performance change と behavior change を分離する方針が確認済みであること
- [x] 使用可能な profiler または sampling tool が 1 つ以上確認済みであること

## 1. Baseline Visibility

- [x] 1.1 `make bench` の current result を `docs/performance.md` に summary として追記する
- [x] 1.2 `make bench-cross-tools-default` の result を取得する
- [x] 1.3 `make bench-cross-tools-common` の result を取得する
- [x] 1.4 `make bench-cross-tools-fix` の result を取得する
- [x] 1.5 missing optional tools の場合は install path と skip reason を記録する

## 2. Profiling

- [x] 2.1 CLI check directory case の hot path を profiler で確認する
- [x] 2.2 API lint large document case の hot path を profiler で確認する
- [x] 2.3 fix workflow case の hot path を profiler で確認する
- [x] 2.4 profile result を `target/` artifact と docs summary に分けて扱う
- [x] 2.5 profiler が使えたため internal timing counters は不要と docs に記録する

## 3. Targeted Optimization

- [x] 3.1 rule registry / metadata construction の不要な clone/allocation を削減する
- [x] 3.2 repeated regex compilation が残っていないか検出する
- [x] 3.3 config load/validation の repeated work を削減する
- [x] 3.4 line scanning で不要な allocation を削減する
- [x] 3.5 fix loop の duplicate lint を追加で削減できるか検証する

## 4. Regression Visibility

- [x] 4.1 benchmark report に git commit / version を含める
- [x] 4.2 performance docs に before/after table を追加する
- [x] 4.3 perf baseline refresh の判断基準を明記する
- [x] 4.4 behavior tests と benchmark の両方を release 前 checklist に入れる

## Verification

- [x] `make bench` が成功する
- [x] `make perf-check` が成功する
- [x] `make bench-cross-tools-common` が成功または skip reason を出す
- [x] `cargo test --workspace` が成功する
- [x] `make check` が成功する
- [x] `git diff --check` が成功する

## Definition of Done

- [x] current baseline と before/after が docs で比較できること
- [x] 少なくとも 1 つの hot path optimization が behavior-preserving に入っていること
- [x] cross-tool benchmark が再現可能な手順として残っていること
- [x] 改善が出なかった場合も、測定結果と次の仮説が docs に残っていること
