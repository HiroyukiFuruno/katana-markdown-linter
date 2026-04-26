# Tasks

## Definition of Ready

- [x] `v0.12.1` の GitHub Release と crates.io publish が完了していること
- [x] `make release-verify VERSION=v0.12.1` が成功していること
- [x] release 後の branch hygiene が完了していること
- [x] `main` から `release/v0.12.2` 作業ブランチを開始していること
- [x] `v0.12.2` は機能追加ではなく品質改善 patch として扱う方針が確認済みであること

## 0. Issue Inventory

- [x] 0.1 現在の CI workflow、release-preflight、release workflow の job / step / cache 差分を一覧化する
- [x] 0.2 Windows 未検証で壊れ得る箇所を shell、path、`.exe`、Makefile、Bash script、coverage に分けて記録する
- [x] 0.3 誤検知リスクのある rule を `bug`、`test-gap`、`design-debt` に分類する
- [x] 0.4 `v0.12.2` に含める修正と後続 change に回す修正を切り分ける
- [x] 0.5 棚卸し結果を tasks に追記し、以後の実装順に反映する

### Issue Inventory Notes

- `ci-gap`: 通常 CI は macOS / Ubuntu のみだった。`v0.12.2` では `windows-latest` を追加し、Windows では `cargo check --workspace --locked`、`cargo fmt --all -- --check`、`cargo test --workspace --locked` までを required CI とする。
- `ci-gap`: release workflow は crates.io publish を含むため Ubuntu 単独のままにする。Windows compatibility は通常 CI / preflight の責務とし、publish job を OS matrix 化しない。
- `ci-gap`: 通常 CI は手書き `actions/cache`、release-preflight / release は `Swatinem/rust-cache` だった。通常 CI も `Swatinem/rust-cache` に寄せ、workflow ごとの `shared-key` を明示する。
- `ci-gap`: `make action-smoke` と `make mcp-stdio-smoke` は Bash、`bin/<name>` path、install root 前提を持つため Windows へ広げない。`.exe` suffix を含む smoke test 移植は後続 change に回す。
- `ci-gap`: 変更前 baseline は GitHub Actions run `24942241092`（2026-04-25、main push）で、Ubuntu job は 115 秒、macOS job は 93 秒だった。
- `ci-gap`: `v0.12.2` PR では Windows CI run `24944611172` が CRLF による `cargo fmt --check` 失敗、run `24945009045` が Windows short path と `.cmd` 未対応の test portability gap で失敗した。`.gitattributes` と Windows 用 benchmark mock により run `24945063974` の Windows job は 2 分 11 秒で成功した。
- `ci-cache`: 変更後 run `24945063974` は macOS 43 秒（full cache hit、約 106 MB）、Ubuntu 1 分 3 秒（full cache hit、約 253 MB）、Windows 2 分 11 秒（cache miss、約 54.5 MB save）だった。release-preflight run `24945063970` は 1 分 19 秒（full cache hit、約 342 MB）だった。
- `bug`: `MD014`、`MD029`、`MD034`、`MD055`、`MD056`、`MD058`、`MD059` の既知誤検知は rule-local test と file-level regression で固定済み。
- `test-gap`: mixed Markdown fixture は HTML badge、inline code、math、backtick / tilde fence、Mermaid pipe、reference link、nested ordered list を含む。
- `design-debt`: `MD034` と `MD059` には line-local な inline code / HTML 判定が残る。現時点では回帰 fixture で固定し、より深い token parser 化は後続に回す。

## 1. Windows CI

- [x] 1.1 CI matrix に `windows-latest` を追加する
- [x] 1.2 Windows で実行する command を `cargo check` / `cargo test --workspace --locked` 中心に固定する
- [x] 1.3 Windows で `make` target を実行する場合は shell と prerequisite を明示する
- [x] 1.4 `kml` binary の `.exe` suffix を install / smoke test で扱えるようにする
- [x] 1.5 Windows job が失敗した場合、原因を workflow log から分類し、test-gap か portability bug として tasks に反映する

## 2. CI/CD Cache

- [x] 2.1 通常 CI の手書き `actions/cache` と release/preflight の `Swatinem/rust-cache` を比較する
- [x] 2.2 cache key が OS、lockfile、feature set、toolchain に対して安全か確認する
- [x] 2.3 cache hit/miss と job duration を変更前後で記録する
- [x] 2.4 採用する cache strategy を workflow 全体で統一する
- [x] 2.5 AST lint または workflow test で、CI / preflight / release の cache 方針が意図せず乖離しないようにする

## 3. False-Positive Regression Corpus

- [x] 3.1 ファイル単位 fixture に backtick / tilde fence、Mermaid pipe、HTML badge、inline code、math、reference link、nested list を含める
- [x] 3.2 `MD014`、`MD029`、`MD034`、`MD055`、`MD056`、`MD058`、`MD059` の誤検知ケースを rule-local test と file-level test の両方で固定する
- [x] 3.3 単一行判定が残っている rule を `DocumentContext` 利用状況で棚卸しする
- [x] 3.4 code block / inline code / HTML / table / reference context を無視している rule を優先修正する
- [x] 3.5 修正後に `make dogfood` の baseline diagnostics が意図せず増えていないことを確認する

## 4. Self-Review And Scope Control

- [x] 4.1 `v0.12.2` の修正が patch release として妥当か確認する
- [x] 4.2 破壊的な CLI / API / config 変更が含まれていないことを確認する
- [x] 4.3 後続に回す課題を `active-roadmap.md` または別 change に残す
- [x] 4.4 README / docs を変更する場合は英語のみで記述し、`make ast-lint` を通す
- [x] 4.5 release 前に changelog を `v0.12.2` に更新する

## Verification

- [x] `make fmt-check`
- [x] `make lint`
- [x] `make ast-lint`
- [x] `cargo test --workspace --locked`
- [x] `cargo test --all-features --locked`
- [x] `make dogfood`
- [x] Windows CI job が成功していること
- [x] `make release-check VERSION=v0.12.2`
- [x] `git diff --check`

## Definition of Done

- [x] Windows が release 前の required CI で検証されていること
- [x] CI / preflight / release の cache strategy が説明可能であること
- [x] 誤検知しやすい mixed Markdown fixture が file-level regression として固定されていること
- [x] 単一行判定に起因する残課題が可視化されていること
- [x] `v0.12.2` に含めない課題が後続 change または roadmap に残っていること
