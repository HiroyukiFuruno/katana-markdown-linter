## Definition of Ready

- [ ] `v0.12.9` の公開向け gate が完了していること
- [ ] `v0.12.11` は性能最適化ではなく品質定着 patch であることに合意済みであること
- [ ] `precision-first` 方針（未解決の true-positive/false-negative がないこと）を開始前に確認していること
- [ ] `v0.12.9` の public confidence score 100点満点 / threshold 90 / hard blocker 0 件ルールを再適用することに同意していること

## 1. Internal Quality Hardening

- [ ] 1.1 `src/cli.rs` と関連 CLI ファイルの責務分離方針を確定する（input 展開 / validation / execution / 出力整形）
- [ ] 1.2 `check`・`fix`・`fmt` の内部実行フローを分割しても、json schema、file ordering、exit code、診断 order が変わらないことをテストで固定する
- [x] 1.3 `make fmt-check` / `make lint` / `make ast-lint` / `cargo test --workspace --locked` の基本回帰に加え、CLI 契約テスト (`cli_convergence_contract`, `cli_path_context_contract`) を必須化する
  - `make fmt-check` / `make lint` / `cargo test --workspace --locked` をこの作業セッションで再実行し、すべて pass
- [x] 1.4 `scripts/ci/internal-quality.py`（新規）を追加し、`target/internal-quality-report.json` に `src` 主要ファイルの LOC、サイズ上位、責務重複候補を出力する
- [x] 1.5 新規 `make internal-quality-check` を追加し、内部品質 evidence を 1 回目の再現可能データとして残す
- [ ] 1.6 `internal-quality` の改善対象（対象ファイル・対象関数）は、v0.12.11 外付けの次 version へ先送りするべきものを明確化して分類する

## 2. External Quality Hardening

- [x] 2.1 `tests/fixtures/public-confidence/corpus` を 1 ファイルから最小代表複数ファイルに拡張し、fenced code（```/~~~）、table、inline code、HTML、reference、混在日本語/英語を網羅する
- [x] 2.2 public confidence の結果分類ルールを定義し、classification の未分類をエラー扱いにする
- [x] 2.3 `scripts/ci/public-confidence.py` / `public_confidence_runner.py` を更新し、分類付き診断情報を `target/public-confidence-report.json` に保持する
- [x] 2.4 `make public-confidence` / `make external-katana-dogfood` 実行時に、finding 分類と release blocker をレポートとして残し、未分類カテゴリを残さない運用にする
- [x] 2.5 `make public-confidence` の結果を `public-confidence` と `external confidence` の比較軸で整理し、follow-up として残す
  - curated は 2 ファイル・995 バイト・4 件 true-positive・release blocker 0（`target/public-confidence-report-curated.json`）
  - KatanA external は 31 ファイル・196622 バイト・4 件 true-positive（いずれも md-broken-link）・release blocker 0（`target/public-confidence-report-katana.json`）

## 3. Performance Regression Control

- [x] 3.1 `scripts/ci/perf-check.py` に `--strict` と `--max-ratio`（または同等の比率閾値）を追加する
- [x] 3.2 `make perf-check` は既存どおり情報収集を継続しつつ、`make perf-check-strict`（新規）として性能退行を fail 可能にする
- [x] 3.3 `api_lint_inline_code_heavy_document` と `api_format_large_document` を重点監視 case とし、退行時は原因説明を tasks に残す
  - `make perf-check-strict --max-ratio 1.4` では `api_lint_inline_code_heavy_document` が `1.05x`、`api_format_large_document` が `1.07x`（いずれも閾値内）
- [x] 3.4 性能改善作業はまず `make perf-check` / `make perf-check-strict` / `make bench` の結果を snapshot と比較し、precision regression check が先に通っていることを確認してから実施する
  - `precision` 側は `cargo test --test cli_convergence_contract` と `cargo test --test cli_path_context_contract` を先行実行し、合格後に性能指標を比較する
- [ ] 3.5 baseline 更新は説明不能な回帰を消す根拠付きレビュー後にのみ `make perf-refresh-baseline` を実行する

## 4. Quality Scoring

- [x] 4.1 `v0.12.9` の public confidence score と同等の形式（カテゴリ・total・threshold・verification）で、`v0.12.11` 向けの `public-confidence-score.json` を定義する
- [x] 4.2 score のカテゴリは、`External corpus confidence` / `Precision regression` / `Command convergence` / `Performance stability` / `Release reproducibility` の 5軸を採用し、配点は `30/25/20/15/10` とする
- [x] 4.3 `v0.12.11` の検証結果（カテゴリ別根拠・再現手順・verification）を上記 JSON に反映する
- [x] 4.4 合否ルール（`score >= 90` かつ `technical_hard_blockers` 0 件）を明記し、残存する known limitation / follow-up を tasks に残す
- [x] 4.5 `v0.12.11` の quality score と public summary をユーザー提示用に整える

## Verification

- [x] `make ast-lint`（`make release-check` 経由）
- [x] `cargo test --all-features --locked`（`make release-check` 経由）
- [x] `make perf-check`
- [x] `make perf-check-strict`（追加）
- [x] `make public-confidence`（`PUBLIC_CONFIDENCE` 追加）
- [x] `make internal-quality-check`（新規）
- [x] `public-confidence-score.json`（作成・埋め込み）
- [x] `make coverage-blocking`
- [x] `make release-check VERSION=v0.12.11`
- [x] `cargo test --test public_confidence_contract --locked`
- [x] `cargo test --test cli_convergence_contract --locked`
- [x] `cargo test --test cli_path_context_contract --locked`
- [x] `make fmt-check`
- [x] `make lint`
- [x] `cargo test --workspace --locked`
- [x] `git diff --check`

## Definition of Done

- [x] 内部品質 evidence（`target/internal-quality-report.json`）が再現可能に出力され、主要責務分離対象が記録されている
- [x] public-confidence が corpus 拡張と finding 分類を満たし、未分類が残らない状態で実行できる
- [x] 性能退行 strict gate がルール化され、監視対象 case の regression 理由が tasks に残る
- [x] 既存の precision / convergence テストが通るまま、`v0.12.11` の quality hardening を出荷できる状態になっている
- [x] `release-check` 前提の検証が通過する
