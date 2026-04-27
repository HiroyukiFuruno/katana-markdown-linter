# Design for v0.12.12 Quality & Performance Hardening

## 1. 品質ゲートの強化 (Quality Gate Overhaul)
<<<<<<< HEAD

=======
>>>>>>> origin/main
リファクタリングに着手する前の**最初のステップ**として、品質ゲート（特に `ast-linter`）の検証対象を堅牢化します。
現状の `tests/ast_linter.rs` は `read_workspace_file("src/cli.rs")` のように特定ファイルの存在や中身をハードコードで前提としており、責務分割でファイルが移動すると検証がすり抜ける（または誤検知する）脆さがあります。
これを `scan_rust_sources` などを活用して **`src/` 配下全体を例外なく走査** する仕組みに改修し、「どこに移動してもルールが適用される」状態を確立してからコードの分割に臨みます。

## 2. 内部品質 (Internal Quality)

### 2.1 `src/cli.rs` の責務分離
<<<<<<< HEAD

現状の `src/cli.rs` は約2400行の巨大なファイルであり、引数パース、入力ファイルの探索、バリデーション、Linter/Formatter/Fixerの呼び出し、及び結果の出力（標準出力・JSON形式など）がすべて密結合しています。
これを以下のレイヤーに分離します。

=======
現状の `src/cli.rs` は約2400行の巨大なファイルであり、引数パース、入力ファイルの探索、バリデーション、Linter/Formatter/Fixerの呼び出し、及び結果の出力（標準出力・JSON形式など）がすべて密結合しています。
これを以下のレイヤーに分離します。
>>>>>>> origin/main
- **Arguments & Parsing Layer**: `Cli` 構造体の定義と、CLI引数の展開
- **Input Resolution Layer**: `expand_inputs`, `filter_paths` など、対象ファイルの列挙・除外処理
- **Execution Workflow Layer**: `run_check_like`, `run_fmt`, `run_stdin_*` などの実行フロー制御
- **Reporter Layer**: `output_report`, `render_text_report` など、診断結果のフォーマットとコンソール出力処理

### 2.2 `target/internal-quality-report.json` 指摘事項の対応
<<<<<<< HEAD

=======
>>>>>>> origin/main
`make internal-quality-check` で特定された `split_candidates` に対処し、関数の肥大化や重複ロジックを整理します。

## 3. パフォーマンス向上 (Performance)

### 3.1 メモリアロケーションの削減
<<<<<<< HEAD

事前の解析により、ホットパスにおいて不要な `.clone()`, `.to_string()`, `.collect::<Vec<_>>()` が散見されます。

=======
事前の解析により、ホットパスにおいて不要な `.clone()`, `.to_string()`, `.collect::<Vec<_>>()` が散見されます。
>>>>>>> origin/main
- 文字列や配列の所有権が不要な箇所では、参照 (`&str`, `&[T]`) を活用。
- イテレータチェーンの途中で `Vec` に `collect` せず、遅延評価のまま処理を続ける設計に変更。

### 3.2 重点監視ケースのパフォーマンス改善
<<<<<<< HEAD

=======
>>>>>>> origin/main
`v0.12.11` での計測対象であった `api_lint_inline_code_heavy_document` や `api_format_large_document` などを中心に、AST探索・評価時のコストを低減します。改善後は `make perf-refresh-baseline` を行い、より厳しい基準で今後の退行を防ぎます。

## 4. 外部品質 (External Quality)

### 4.1 Known Limitations の整理と対策
<<<<<<< HEAD

`v0.12.11` の `public-confidence` で検出されていた `md-broken-link` (4件) などについて、これが Linter の制約として許容すべき（外部依存による真のリンク切れ）か、それとも無視ルールの拡張によって運用でカバーすべきかを決定し、結果分類の精度をさらに高めます。
=======
`v0.12.11` の `public-confidence` で検出されていた `md-broken-link` (4件) などについて、これが Linter の制約として許容すべき（外部依存による真のリンク切れ）か、それとも無視ルールの拡張によって運用でカバーすべきかを決定し、結果分類の精度をさらに高めます。
>>>>>>> origin/main
