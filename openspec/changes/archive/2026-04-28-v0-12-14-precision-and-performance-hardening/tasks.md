## Definition of Ready

- [x] `v0.12.13` の PR がマージ済み・タグ済みであること
- [x] `release/v0.12.14` ブランチが main から分岐していること
- [x] 本バージョンが「機能追加ゼロ」の性能特化リリースであることの合意
- [x] `precision-first` 誓約: 一切の挙動退行・精度退行を許容しない

## 1. F-A: `line_in_blocks()` → `code_line_flags` 参照化 (Phase 1)

*目的: 全 4 inline extractor の O(L×b) code block 線形走査を O(L) 索引参照に置き換える*

- [x] 1.1 `src/rules/markdown/inline/code_spans.rs` の `extract_inline_code_spans` 引数を `code_blocks: &[BlockRange]` から `code_line_flags: &[bool]` に変更し、`line_in_blocks()` 呼び出しを `code_line_flags.get(idx).copied().unwrap_or(false)` に置き換える
- [x] 1.2 `src/rules/markdown/inline/html.rs` の `extract_inline_html_elements` を同様に変更する
- [x] 1.3 `src/rules/markdown/inline/links/mod.rs` の `extract_inline_links` を同様に変更する
- [x] 1.4 `src/rules/markdown/inline/reference_definitions.rs` の `extract_reference_definitions` を同様に変更する
- [x] 1.5 `src/rules/markdown/document.rs` の各 OnceLock 初期化コード (`get_or_init` の引数) を `&self.code_blocks` → `&self.code_line_flags` に変更する
- [x] 1.6 `src/rules/markdown/inline/scan.rs` の `line_in_blocks` が他から使われていないことを確認し、不要なら削除する
- [x] 1.7 `cargo check` でコンパイルが通ることを確認する

## 2. F-B: `inside_code_span()` → `partition_point` 二分探索 (Phase 2)

*目的: 文字位置ごとの O(s) 線形走査を O(log s) 二分探索に置き換える*

- [x] 2.1 `src/rules/markdown/inline/scan.rs` の `inside_code_span()` を `partition_point` を使った実装に書き換える (document.rs:220-228 の `is_inside_inline_code` パターンを踏襲)
- [x] 2.2 `line_index` パラメータが不要になる場合は呼び出し元 (html.rs, links/mod.rs) と合わせて削除する。互換性ビルドが難しければ `_line_index` で残す
- [x] 2.3 `cargo check` でコンパイルが通ることを確認する

## 3. F-C: backtick marker String アロケーション除去 (Phase 3)

*目的: `"`".repeat(marker_len)` の per-span String 確保をゼロアロケーションに置き換える*

- [x] 3.1 `src/rules/markdown/inline/code_spans.rs` に `find_closing_marker(text: &str, start: usize, marker_len: usize) -> Option<usize>` を追加する
- [x] 3.2 `"`".repeat(marker_len)` + `line.text[..].find(&marker)` を `find_closing_marker` 呼び出しに置き換える
- [x] 3.3 `cargo check` でコンパイルが通ることを確認する

## 4. Quality Gates (Phase 4)

- [x] 4.1 `make ast-lint` を実行し、全テスト pass を確認する
- [x] 4.2 `cargo test --all-features --locked` を実行し、全スイート pass を確認する
- [x] 4.3 `cargo test --test cli_convergence_contract --locked` pass 確認
- [x] 4.4 `cargo test --test cli_path_context_contract --locked` pass 確認
- [x] 4.5 `cargo test --test public_confidence_contract --locked` pass 確認
- [x] 4.6 `make public-confidence` を実行し、`unclassified_count: 0` / `release_blocking_issues: []` を確認する
- [x] 4.7 `make internal-quality-check` を実行し、スコア悪化がないことを確認する
- [x] 4.8 `make coverage-blocking` を実行し、uncovered ≤ baseline を確認する
- [x] 4.9 `make perf-check-strict` を実行し ratio ≤ 1.40x を確認、改善が確認できた場合のみ `make perf-refresh-baseline` でベースライン更新する
- [x] 4.10 `public-confidence-score.json` を作成し 100/100 を反映する

## Verification

- [x] `make ast-lint`
- [x] `cargo test --all-features --locked`
- [x] `cargo test --test public_confidence_contract --locked`
- [x] `cargo test --test cli_convergence_contract --locked`
- [x] `cargo test --test cli_path_context_contract --locked`
- [x] `make perf-check-strict`
- [x] `make public-confidence`
- [x] `make internal-quality-check`
- [x] `make coverage-blocking`
- [x] `make release-check VERSION=v0.12.14`

## Definition of Done

- [x] 全 4 inline extractor が `code_line_flags` 参照に切り替わっている
- [x] `inside_code_span()` が `partition_point` を使った O(log s) 実装になっている
- [x] `"`".repeat()` が `find_closing_marker()` に置き換わっている
- [x] 全 inline 系診断の件数・range が変更前後で完全一致している（精度退行ゼロの証明）
- [x] `make perf-check-strict` が ratio ≤ 1.40x で通過している
- [x] `public-confidence-score.json` が作成され 100/100 を満たしている
- [x] `make release-check VERSION=v0.12.14` がエラーなく通過する
