## Why

`v0.12.13` で `src/cli/workflow.rs` の責務分割と `md059::normalize_link_text` の中間 `Vec` 除去を完了し、public-confidence-score 100/100 を維持しました。
一方、ルール共通の hot path には依然として削減可能なコストが残っています。

1. `src/rules/markdown/document.rs` の inline 抽出 (`inline_code_spans` / `inline_html_elements` / `inline_links` / `reference_definitions`) は OnceLock により 1 度だけ計算されるものの、各メソッドが独立に line ループを行うため、構造的には同一の走査が 4 回実行されています。
2. `src/rules/markdown/rules/md051.rs` は `config.properties.ignored_pattern` を `evaluate_context` 呼び出しごとに `regex::Regex::new` でコンパイルしています。複数ファイル / 複数ルール実行時にコンパイルコストが線形に積み上がります。
3. `src/rules/markdown/rules/md046.rs` の一部判定が `code_blocks.iter().any()` の O(n) 走査に依存しており、`DocumentContext::code_line_flags` (O(1) 索引) を活用できていません。

`v0.12.14` は、`v0.12.13` までに確立した「precision-first」誓約と全品質ゲートを維持しながら、上記 3 つの hot path を統合・キャッシュ化・索引化することで、検査精度を一切退行させずに ratio ベースラインを引き下げることを目的とします。

加えて、未着手の精度 fix+ 拡充候補 (P-1〜P-8) を `active-roadmap.md` に登録し、`v0.12.15` 以降で逐次消化する方針を明記します。

## What Changes

- パフォーマンス: `DocumentContext` に inline 要素を 1 パスで抽出する `InlineIndex` を導入し、既存の `inline_code_spans` / `inline_html_elements` / `inline_links` / `reference_definitions` メソッドの内部実装を委譲する。外部シグネチャは維持。
- パフォーマンス: `md051` の `ignored_pattern` を rule 構築時に 1 度だけコンパイルし、`evaluate_context` ループで再利用する。
- パフォーマンス: `md046` 内の `code_blocks.iter().any()` 走査を `DocumentContext::code_line_flags` 参照に置き換える。
- ロードマップ: `openspec/changes/active-roadmap.md` に `v0.12.14` の行追加と、未着手の精度 fix+ 候補 (MD052 / MD046 / MD043 / MD056 / MD034 / MD051 / MD059 / MD013) を後続バージョンに割り付け。
- 品質: `public-confidence-score.json` を v0.12.14 用に再作成し 100/100 を継続。

## Capabilities

### New Capabilities

- なし（既存挙動の高速化のみ）

### Modified Capabilities

- `performance-regression-control`: inline 抽出統合、regex キャッシュ、code-line 索引活用による hot path コスト削減
- `internal-quality-hardening`: 重複走査の排除によりルール間の依存構造をシンプル化

## Impact

- 変更ファイル:
  - `src/rules/markdown/document.rs` (InlineIndex 追加・既存メソッドの内部委譲化)
  - `src/rules/markdown/rules/md051.rs` (regex プリコンパイル)
  - `src/rules/markdown/rules/md046.rs` (code_line_flags 参照化)
  - `openspec/changes/active-roadmap.md` (バージョン行と精度 fix+ ロードマップ追記)
- 外部 API / CLI 契約 / 設定スキーマには変更なし。
- 既存の `cli_convergence_contract` / `cli_path_context_contract` / `public_confidence_contract` / `tests/ast_linter.rs` は引き続き全 pass させる。

## Non-Goals

- 新ルール追加・既存ルールの検査ロジック変更（精度退行リスク回避）
- 精度 fix+ 拡充 (P-1〜P-8) の本リリースでの実装（roadmap 登録のみ、実装は v0.12.15 以降）
- MD013 wrap fix (P-8) の本リリースでの議論（unsafe-fix 扱いが妥当なため別 change で扱う）
- 設定ファイルや CLI 引数の追加
