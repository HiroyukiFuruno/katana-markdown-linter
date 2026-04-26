# Tasks

## Definition of Ready

- `v0.12.3` の release と archive が完了している。
- `main` が release 後の基準になっている。
- `v0.12.4` は展開系ではなく、`linter` / `check` / `fix` / `fmt` の中核品質を優先する。
- MCP Registry / Hub 公開、MCP package artifact、遠隔 MCP 接続（remote MCP transport）は後続 change のまま維持する。

## 0. Issue Inventory

- [x] `linter` / `check` / `fix` / `fmt` の既知課題を `bug`、`test-gap`、`perf-risk`、`design-debt`、`ci-gap` に分類する。
- [x] 誤検知、検出漏れ、安全修正、formatter scope、CLI exit/output、速度、テスト不足を同じ棚卸しに含める。
- [x] `v0.12.4` で対応するものと後続 change に送るものを分け、後続対象を tasks に残す。
- [x] 進行中に OpenSpec task 外の高リスク不足が見つかった場合だけ、作業を中断してユーザー判断を仰ぐ。

## 1. CLI Contract Hardening

- [x] `kml check` が対象ファイルを書き換えないことを CLI 結合テストで固定する。
- [x] `kml check --fix` が default-safe fix、fix 後再診断、残存違反報告を行うことを固定する。
- [x] `kml fix` が明示的な safe fix command として applied fixes / remaining diagnostics を返すことを固定する。
- [x] `kml fmt` を lint fix の別名ではなく、formatter contract として固定する。
- [x] text output と JSON output が同じ stdout payload に混在しないことを確認する。
- [x] stdin/stdout 利用時の `check` / `fix` / `fmt` の期待動作を固定する。

## 2. Linter / Fix Precision

- [x] 対象 rule ごとに最小再現の rule-local test を追加する。
- [x] mixed Markdown document fixture に誤検知・検出漏れの再発条件を追加する。
- [x] fix candidate の重複、隣接、競合、順序依存をテストで固定する。
- [x] `fix` と `check --fix` の再実行で不要な差分が増えないことを確認する。
- [x] unsafe fix が既定動作に混ざっていないことを確認する。

## 3. Formatter Hardening

- [x] `fmt` の対象範囲を、空行、末尾改行、リスト周辺、table 周辺などの formatter 責務として整理する。
- [x] `fmt` の冪等性（idempotence）を fixture で固定する。
- [x] CRLF、最終行、空ファイル、front matter、code block 周辺の境界を検証する。
- [x] `fmt` が lint fix や意味変更を行わないことを確認する。
- [x] editor integration を想定した stdin/stdout の成功・失敗 contract を確認する。

## 4. Performance and Quality Gates

- [x] 実装前に `make perf-check` または同等の基準値を記録する。
- [x] `check` / `fix` / `fmt` / directory check / diagnostics-heavy corpus の代表経路を測定対象に含める。
- [x] 速度改善を行った場合、diagnostics、fixed content、formatted content、exit code、JSON shape が変わっていないことを確認する。
- [x] dogfood で中核コマンドの使い勝手と release 前の確認手順を検証する。
- [x] Windows CI で release 前に必要な `cargo check`、`cargo fmt`、`cargo test` の責務を確認する。

## 5. Release Preparation

- [x] `v0.12.4` の version bump を行う。
- [x] `CHANGELOG.md` に `linter` / `check` / `fix` / `fmt` の中核改善として記載する。
- [x] 必要に応じて README / docs を更新し、英語で記述する。
- [x] OpenSpec delta を main specs に同期し、完了後に change を archive する。
- [x] release branch / PR / GitHub Release / crates.io publish の release flow を `/impl-release` workflow へ引き渡す。
- [x] release 後に branch hygiene を行い、ローカルブランチと worktree の残存理由を報告する。

## Verification

- [x] `make fmt-check`
- [x] `make lint`
- [x] `make ast-lint`
- [x] `cargo test --workspace --locked`
- [x] `cargo test --workspace --all-features --locked`
- [x] `make dogfood`
- [x] `make perf-check`
- [x] `make release-check VERSION=v0.12.4`
- [x] GitHub Actions required CI を release PR で確認する
- [x] `git diff --check`

## Evidence

- `tests/cli_core_contract.rs` で `check` の no-write、`check --fix` の JSON command identity、`fix` の safe-only behavior、`fmt` の formatter-only behavior、stdin config validation を固定した。
- `src/cli.rs` で `check --fix` の report command を `check` として維持し、stdin の config validation と JSON fix report を追加した。
- `examples/perf_benchmark.rs` と `scripts/ci/perf-check.py` に `api_format_large_document`、`cli_fix_many_small_files`、`cli_fmt_many_small_files` を追加し、`tests/fixtures/perf-baseline.json` を更新した。
- `make fmt-check`、`make lint`、`make ast-lint`、`cargo test --workspace --locked`、`cargo test --workspace --all-features --locked`、`make dogfood`、`make perf-check`、`make release-check VERSION=v0.12.4`、`git diff --check` が通過した。
- GitHub Actions required CI は release PR 段階の最終 gate として `/impl-release` workflow で確認する。

## Definition of Done

- `check` / `fix` / `fmt` の責務が仕様・実装・テストで一致している。
- 誤検知、検出漏れ、安全修正、整形の対象範囲が回帰テストで固定されている。
- 速度改善が正しさを崩していないことが証拠付きで確認されている。
- `v0.12.4` の release が完了し、配布展開は `v0.13.0` 以降へ戻せる状態になっている。
