# Tasks

## Definition of Ready

- [x] `v0.12.2` の GitHub Release と crates.io publish が完了していること
- [x] `make release-verify VERSION=v0.12.2` が成功していること
- [x] `v0.12.2` 公開（release）後のブランチ整理（branch hygiene）が完了していること
- [x] `main` から `release/v0.12.3` 作業ブランチを開始していること
- [x] `v0.12.3` は新機能ではなく精度・速度・テスト改善パッチ（patch）として扱う方針が確認済みであること

## 0. Issue Inventory

- [ ] 0.1 誤検知、検出漏れ、速度劣化、test-gap を `bug` / `test-gap` / `design-debt` / `perf-risk` に分類する
- [ ] 0.2 `MD034`、`MD059`、インラインコード（inline code）、HTML、reference、command prompt 周辺の残課題を棚卸しする
- [ ] 0.3 単体テスト（UT）と結合テスト（IT）の不足をテスト層（test layer）ごとに整理する
- [ ] 0.4 `v0.12.3` に含める修正と後続 change に回す修正を切り分ける
- [ ] 0.5 棚卸し結果を tasks に追記し、以後の実装順に反映する

## 1. 精度強化（Precision Hardening）

- [ ] 1.1 修正対象 rule の最小再現をルール単位テスト（rule-local test）に追加する
- [ ] 1.2 文書単位の混在 fixture（document-level mixed fixture）に再発条件を追加する
- [ ] 1.3 `DocumentContext` を使うべき判定とルール単位（rule-local）で十分な判定を分ける
- [ ] 1.4 誤検知修正後に unrelated diagnostics が増えていないことを確認する
- [ ] 1.5 修正しない設計負債は後続 change として roadmap に残す

## 2. 速度強化（Performance Hardening）

- [ ] 2.1 実装前に `make bench` または `make perf-check` で基準値（baseline）を記録する
- [ ] 2.2 directory traversal、config validation、DocumentContext、rule dispatch、reporting、fix application の高負荷経路（hot path）を分類する
- [ ] 2.3 測定できる高負荷経路（hot path）だけを最小変更で改善する
- [ ] 2.4 正しさのテスト（correctness test）を通した上で before / after を tasks または docs に記録する
- [ ] 2.5 CI の required gate を不安定な時計時間の閾値（wall-clock threshold）に依存させない

## 3. テスト強化（Test Hardening）

- [ ] 3.1 単体テスト（UT）を rule、config、path、fix idempotence、Windows path 表現に拡充する
- [ ] 3.2 結合テスト（IT）を CLI workspace、fixture corpus、dogfood、cross-tool benchmark、Windows CI 差分に拡充する
- [ ] 3.3 test helper が OS 固有の実行形式を正しく扱うことを固定する
- [ ] 3.4 test の都合だけで商用コードの責務を曲げていないことを確認する
- [ ] 3.5 必要な AST lint または Makefile target を追加し、回帰を機械的に止める

## 4. 公開準備（Release Preparation）

- [ ] 4.1 crate version を `0.12.3` に更新する
- [ ] 4.2 `CHANGELOG.md` に `v0.12.3` を追加する
- [ ] 4.3 public docs を変更した場合は英語のみで記述し、`make ast-lint` を通す
- [ ] 4.4 OpenSpec の task と spec delta を同期し、完了後に archive する
- [ ] 4.5 release 前に `make release-check VERSION=v0.12.3` を通す

## Verification

- [ ] `make fmt-check`
- [ ] `make lint`
- [ ] `make ast-lint`
- [ ] `cargo test --workspace --locked`
- [ ] `cargo test --all-features --locked`
- [ ] `make dogfood`
- [ ] `make perf-check`
- [ ] GitHub Actions の Ubuntu / macOS / Windows CI が成功していること
- [ ] `make release-check VERSION=v0.12.3`
- [ ] `git diff --check`

## Definition of Done

- [ ] 精度改善がルール単位テスト（rule-local test）と文書単位fixture（document-level fixture）の両方で固定されていること
- [ ] 速度改善が基準値（baseline）と before / after で説明できること
- [ ] 単体テスト（UT）と結合テスト（IT）の追加範囲が tasks に残っていること
- [ ] `v0.12.3` に含めない課題が roadmap または後続 change に残っていること
- [ ] 公開前ゲート（release gate）と CI が通っていること
