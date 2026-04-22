# katana-markdown-linter

## Summary

markdownlint 互換のルールと自動修正（fix）機能を備えた、Rust 製の Markdown リンターライブラリクレート。
エディタやツールへのプログラマティックな組み込みを第一級ユースケースとして設計する。

## Problem

Rust エコシステムにおいて、Markdown リンターをライブラリとして組み込める成熟したクレートが存在しない。

- **`rumdl`** (⭐1,100+): CLI 専用。ライブラリ API ドキュメントが存在せず（docs.rs 404）、CLI 向けの重い依存を引きずる。
- **`mkdlint`** (⭐0): ライブラリ API は完璧だが、コミュニティ規模が皆無で将来性リスクが高い。

現在 KatanA エディタでは `katana-linter` 内部に自前の Markdown ルール実装を持っているが、エディタとリンターの責務が混在しており、独立したテスト・メンテナンスが困難。

## Solution

`katana-markdown-linter` を独立した public crate として作成する。

### 設計原則

1. **Library-First**: CLI はオプショナル。ライブラリ API がプライマリインターフェース
2. **markdownlint 互換**: 公式 markdownlint のルール ID（MD001-MD058）との互換性を維持
3. **Fix 機能内蔵**: lint と fix を単一クレートで提供
4. **最小依存**: `pulldown-cmark` ベースの AST パーサのみを必須依存とし、CLI/LSP 関連は feature flags で分離
5. **docs.rs 完全対応**: 全公開 API にドキュメントを付与

### 公開 API の設計方針

```rust
use katana_markdown_linter::{lint, fix, LintOptions, LintResult, FixResult};

// Lint: テキストを受け取り、診断結果を返す
let results: Vec<LintResult> = lint(content, &options)?;

// Fix: テキストを受け取り、修正済みテキストを返す
let fixed: FixResult = fix(content, &options)?;
```

### ルールカバレッジ（初期目標）

- KatanA で実装済みの 24 ルールを移植
- Fix 対応: 最低 80% のルールで自動修正をサポート
- 将来: markdownlint の全 58 ルールをカバー

## Acceptance Criteria

- [ ] `cargo add katana-markdown-linter` でライブラリとして利用可能
- [ ] `lint()` / `fix()` のシンプルな公開 API
- [ ] docs.rs で 100% の API ドキュメンテーション
- [ ] markdownlint 互換のルール ID（MD001-MD058）
- [ ] KatanA エディタからの利用（`katana-linter` が依存として参照）
- [ ] `pulldown-cmark` ベースの AST パーサによる正確なルール評価
- [ ] CI（GitHub Actions）でのテスト・lint・ドキュメント生成
