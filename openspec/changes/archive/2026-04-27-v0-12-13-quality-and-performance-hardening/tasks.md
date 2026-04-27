## Definition of Ready

- [ ] `v0.12.12` の PR がマージ済みであること
- [ ] `release/v0.12.13` ブランチが main から分岐していること
- [ ] 本バージョンが「機能追加ゼロ」の品質・性能特化リリースであることの合意
- [ ] パフォーマンス向上よりも検査精度を絶対優先する「`precision-first`」の誓約（一切の挙動退行を許容しない）
- [ ] `make internal-quality-check` を事前に実行し、リファクタリング前のベースラインスコアを確認済みであること

## 1. Architecture Refactoring (Phase 1)

*目的: 最大の技術的負債である src/cli/workflow.rs を安全に解体し、保守性を高める*

- [ ] 1.1 `src/cli/workflow/common.rs` を作成し、共有型（`UnsafeFixPolicy`, `FixedContent`）と共有関数（`validate_effective_config`, `load_effective_config`, `apply_fixes_until_stable`）を移動する
- [ ] 1.2 `src/cli/workflow/check.rs` を作成し、check/fix コマンドフロー（`run_check_like`, `run_stdin_check_like`, `resolve_unsafe_fix_policy`）を移動する
- [ ] 1.3 `src/cli/workflow/fmt.rs` を作成し、fmt コマンドフロー（`run_fmt`, `run_stdin_fmt`, `format_stdin_content`）を移動する
- [ ] 1.4 `src/cli/workflow/config_cmd.rs` を作成し、rule/config コマンド群（`run_rule`, `run_config`, `render_rule`, `render_config`, `prompt_unsafe_confirmation`）を移動する
- [ ] 1.5 `src/cli/workflow/mod.rs` に `pub use` を集約し、旧 `src/cli/workflow.rs` を削除する
- [ ] 1.6 `make internal-quality-check` を実行し、`src/cli/workflow.rs` が `split_candidates` から消えていること、各サブモジュールが ≤400 スコアであることを確認する

## 2. Performance Optimization (Phase 2)

*目的: 残存するホットパスアロケーションを除去し、性能ベースラインを改善する*

- [ ] 2.1 `src/rules/markdown/rules/md059.rs::normalize_link_text` の `collect::<Vec<_>>()` を除去し、中間 Vec なしで同一出力を実現する
- [ ] 2.2 `cargo test --lib -- rules::markdown::rules::md059` で精度退行がないことを確認する
- [ ] 2.3 `make perf-check-strict` を実行し、ratio ≤ 1.40x であることを確認する
- [ ] 2.4 改善が確認できた場合のみ `make perf-refresh-baseline` を実行してベースラインを更新する

## 3. Quality Gates (Phase 3)

*目的: リリース前の総合品質確認*

- [ ] 3.1 `make ast-lint` を実行し、15 テスト全 pass を確認する
- [ ] 3.2 `cargo test --all-features --locked` を実行し、全スイート pass を確認する
- [ ] 3.3 `cargo test --test cli_convergence_contract --locked` pass 確認
- [ ] 3.4 `cargo test --test cli_path_context_contract --locked` pass 確認
- [ ] 3.5 `cargo test --test public_confidence_contract --locked` pass 確認
- [ ] 3.6 `make public-confidence` を実行し、`unclassified_count: 0` / `release_blocking_issues: []` を確認する
- [ ] 3.7 `make internal-quality-check` を実行し、スコア改善を確認する
- [ ] 3.8 `make coverage-blocking` を実行し、uncovered ≤ baseline を確認する（モジュール分割で行数増加の場合は根拠付きで baseline 更新）
- [ ] 3.9 `public-confidence-score.json` を作成・反映する（5 軸評価、合計 100/100 目標）

## Verification

- [ ] `make ast-lint`
- [ ] `cargo test --all-features --locked`
- [ ] `cargo test --test public_confidence_contract --locked`
- [ ] `cargo test --test cli_convergence_contract --locked`
- [ ] `cargo test --test cli_path_context_contract --locked`
- [ ] `make perf-check-strict`
- [ ] `make public-confidence`
- [ ] `make internal-quality-check`
- [ ] `public-confidence-score.json` の作成・反映
- [ ] `make coverage-blocking`
- [ ] `make release-check VERSION=v0.12.13`

## Definition of Done

- [ ] `src/cli/workflow.rs` が削除され、`src/cli/workflow/` サブモジュール群に置き換わっている
- [ ] 各サブモジュールの size_score が ≤400 であること
- [ ] `md059.rs` の中間 `Vec` アロケーションが除去されていること
- [ ] `make perf-check-strict` が通過していること
- [ ] 全ての精密性・収束性テストが引き続きパスし、精度退行がないことが証明されている
- [ ] public-confidence-score が作成され、リリース基準（90点以上、ブロッカなし）を満たしている
- [ ] `make release-check` がエラーなく通過する
