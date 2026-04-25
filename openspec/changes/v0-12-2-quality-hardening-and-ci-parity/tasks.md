# Tasks

## Definition of Ready

- [x] `v0.12.1` の GitHub Release と crates.io publish が完了していること
- [x] `make release-verify VERSION=v0.12.1` が成功していること
- [x] release 後の branch hygiene が完了していること
- [x] `main` から `release/v0.12.2` 作業ブランチを開始していること
- [x] `v0.12.2` は機能追加ではなく品質改善 patch として扱う方針が確認済みであること

## 0. Issue Inventory

- [ ] 0.1 現在の CI workflow、release-preflight、release workflow の job / step / cache 差分を一覧化する
- [ ] 0.2 Windows 未検証で壊れ得る箇所を shell、path、`.exe`、Makefile、Bash script、coverage に分けて記録する
- [ ] 0.3 誤検知リスクのある rule を `bug`、`test-gap`、`design-debt` に分類する
- [ ] 0.4 `v0.12.2` に含める修正と後続 change に回す修正を切り分ける
- [ ] 0.5 棚卸し結果を tasks に追記し、以後の実装順に反映する

## 1. Windows CI

- [ ] 1.1 CI matrix に `windows-latest` を追加する
- [ ] 1.2 Windows で実行する command を `cargo check` / `cargo test --workspace --locked` 中心に固定する
- [ ] 1.3 Windows で `make` target を実行する場合は shell と prerequisite を明示する
- [ ] 1.4 `kml` binary の `.exe` suffix を install / smoke test で扱えるようにする
- [ ] 1.5 Windows job が失敗した場合、原因を workflow log から分類し、test-gap か portability bug として tasks に反映する

## 2. CI/CD Cache

- [ ] 2.1 通常 CI の手書き `actions/cache` と release/preflight の `Swatinem/rust-cache` を比較する
- [ ] 2.2 cache key が OS、lockfile、feature set、toolchain に対して安全か確認する
- [ ] 2.3 cache hit/miss と job duration を変更前後で記録する
- [ ] 2.4 採用する cache strategy を workflow 全体で統一する
- [ ] 2.5 AST lint または workflow test で、CI / preflight / release の cache 方針が意図せず乖離しないようにする

## 3. False-Positive Regression Corpus

- [ ] 3.1 ファイル単位 fixture に backtick / tilde fence、Mermaid pipe、HTML badge、inline code、math、reference link、nested list を含める
- [ ] 3.2 `MD014`、`MD029`、`MD034`、`MD055`、`MD056`、`MD058`、`MD059` の誤検知ケースを rule-local test と file-level test の両方で固定する
- [ ] 3.3 単一行判定が残っている rule を `DocumentContext` 利用状況で棚卸しする
- [ ] 3.4 code block / inline code / HTML / table / reference context を無視している rule を優先修正する
- [ ] 3.5 修正後に `make dogfood` の baseline diagnostics が意図せず増えていないことを確認する

## 4. Self-Review And Scope Control

- [ ] 4.1 `v0.12.2` の修正が patch release として妥当か確認する
- [ ] 4.2 破壊的な CLI / API / config 変更が含まれていないことを確認する
- [ ] 4.3 後続に回す課題を `active-roadmap.md` または別 change に残す
- [ ] 4.4 README / docs を変更する場合は英語のみで記述し、`make ast-lint` を通す
- [ ] 4.5 release 前に changelog を `v0.12.2` に更新する

## Verification

- [ ] `make fmt-check`
- [ ] `make lint`
- [ ] `make ast-lint`
- [ ] `cargo test --workspace --locked`
- [ ] `cargo test --all-features --locked`
- [ ] `make dogfood`
- [ ] Windows CI job が成功していること
- [ ] `make release-check VERSION=v0.12.2`
- [ ] `git diff --check`

## Definition of Done

- [ ] Windows が release 前の required CI で検証されていること
- [ ] CI / preflight / release の cache strategy が説明可能であること
- [ ] 誤検知しやすい mixed Markdown fixture が file-level regression として固定されていること
- [ ] 単一行判定に起因する残課題が可視化されていること
- [ ] `v0.12.2` に含めない課題が後続 change または roadmap に残っていること
