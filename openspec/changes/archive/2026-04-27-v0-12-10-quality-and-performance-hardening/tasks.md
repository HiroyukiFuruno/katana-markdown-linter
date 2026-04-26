## Definition of Ready

- [ ] `v0.12.9` の公開向け gate が完了していること
- [ ] `v0.12.10` は性能最適化ではなく品質定着 patch であることに合意済みであること
- [ ] `precision-first` 方針（未解決の true-positive/false-negative がないこと）を開始前に確認していること
- [ ] `v0.12.9` の public confidence score 100点満点 / threshold 90 / hard blocker 0 件ルールを再適用することに同意していること

## 1. Internal Quality Hardening

- [ ] 1.1 `src/cli.rs` と関連 CLI ファイルの責務分離方針を確定する（input 展開 / validation / execution / 出力整形）
- [ ] 1.2 `check`・`fix`・`fmt` の内部実行フローを分割しても、json schema、file ordering、exit code、診断 order が変わらないことをテストで固定する
- [ ] 1.3 `make fmt-check` / `make lint` / `make ast-lint` / `cargo test --workspace --locked` の基本回帰に加え、CLI 契約テスト (`cli_convergence_contract`, `cli_path_context_contract`) を必須化する
- [ ] 1.4 `scripts/ci/internal-quality.py`（新規）を追加し、`target/internal-quality-report.json` に `src` 主要ファイルの LOC、サイズ上位、責務重複候補を出力する
- [ ] 1.5 新規 `make internal-quality-check` を追加し、内部品質 evidence を 1 回目の再現可能データとして残す
- [ ] 1.6 `internal-quality` の改善対象（対象ファイル・対象関数）は、v0.12.10 外付けの次 version へ先送りするべきものを明確化して分類する

## 2. External Quality Hardening

- [ ] 2.1 `tests/fixtures/public-confidence/corpus` を 1 ファイルから最小代表複数ファイルに拡張し、fenced code（```/~~~）、table、inline code、HTML、reference、混在日本語/英語を網羅する
- [ ] 2.2 public confidence の結果分類ルールを定義し、classification の未分類をエラー扱いにする
- [ ] 2.3 `scripts/ci/public-confidence.py` / `public_confidence_runner.py` を更新し、分類付き診断情報を `target/public-confidence-report.json` に保持する
- [ ] 2.4 `make public-confidence` / `make external-katana-dogfood` 実行時に、finding 分類と release blocker をレポートとして残し、未分類カテゴリを残さない運用にする
- [ ] 2.5 `make public-confidence` の結果を `public-confidence` と `external confidence` の比較軸で整理し、follow-up として残す

## 3. Performance Regression Control

- [ ] 3.1 `scripts/ci/perf-check.py` に `--strict` と `--max-ratio`（または同等の比率閾値）を追加する
- [ ] 3.2 `make perf-check` は既存どおり情報収集を継続しつつ、`make perf-check-strict`（新規）として性能退行を fail 可能にする
- [ ] 3.3 `api_lint_inline_code_heavy_document` と `api_format_large_document` を重点監視 case とし、退行時は原因説明を tasks に残す
- [ ] 3.4 性能改善作業はまず `make perf-check` / `make perf-check-strict` / `make bench` の結果を snapshot と比較し、precision regression check が先に通っていることを確認してから実施する
- [ ] 3.5 baseline 更新は説明不能な回帰を消す根拠付きレビュー後にのみ `make perf-refresh-baseline` を実行する

## 4. Quality Scoring

- [ ] 4.1 `v0.12.9` の public confidence score と同等の形式（カテゴリ・total・threshold・verification）で、`v0.12.10` 向けの `public-confidence-score.json` を定義する
- [ ] 4.2 score のカテゴリは、`External corpus confidence` / `Precision regression` / `Command convergence` / `Performance stability` / `Release reproducibility` の 5軸を採用し、配点は `30/25/20/15/10` とする
- [ ] 4.3 `v0.12.10` の検証結果（カテゴリ別根拠・再現手順・verification）を上記 JSON に反映する
- [ ] 4.4 合否ルール（`score >= 90` かつ `technical_hard_blockers` 0 件）を明記し、残存する known limitation / follow-up を tasks に残す
- [ ] 4.5 `v0.12.10` の quality score と public summary をユーザー提示用に整える

## Verification

- [ ] `cargo test --workspace --locked`
- [ ] `cargo test --all-features --locked`
- [ ] `make fmt-check`
- [ ] `make lint`
- [ ] `make ast-lint`
- [ ] `cargo test --test cli_convergence_contract --locked`
- [ ] `cargo test --test cli_path_context_contract --locked`
- [ ] `make perf-check`
- [ ] `make perf-check-strict`（追加）
- [ ] `make public-confidence`（`PUBLIC_CONFIDENCE` 追加）
- [ ] `make internal-quality-check`（新規）
- [ ] `public-confidence-score.json`（作成・埋め込み）
- [ ] `make coverage-blocking`
- [ ] `git diff --check`

## Definition of Done

- [ ] 内部品質 evidence（`target/internal-quality-report.json`）が再現可能に出力され、主要責務分離対象が記録されている
- [ ] public-confidence が corpus 拡張と finding 分類を満たし、未分類が残らない状態で実行できる
- [ ] 性能退行 strict gate がルール化され、監視対象 case の regression 理由が tasks に残る
- [ ] 既存の precision / convergence テストが通るまま、`v0.12.10` の quality hardening を出荷できる状態になっている
- [ ] `release-check` 前提の検証が通過する
