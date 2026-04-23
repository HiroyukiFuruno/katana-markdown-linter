# katana-markdown-linter

## Summary

markdownlint 互換のルールと自動修正（fix）機能を備えた、Rust 製の Markdown リンターライブラリクレート。
エディタやツールへのプログラマティックな組み込みを第一級ユースケースとして設計する。

## Problem

Rust エコシステムにおいて、Markdown リンターをライブラリとして組み込める成熟したクレートが存在しない。

- **`rumdl`** (⭐1,100+): CLI 専用。ライブラリ API ドキュメントが存在せず（docs.rs 404）、CLI 向けの重い依存を引きずる。
- **`mkdlint`** (⭐0): ライブラリ API は完璧だが、コミュニティ規模が皆無で将来性リスクが高い。

Markdown linter を利用するアプリケーション側と、markdownlint-compatible な rule engine 側の責務は分離されているべきである。
この crate は利用側アプリケーションを知らない共通ライブラリとして、独立したテスト・メンテナンスを可能にする。

## Solution

`katana-markdown-linter` を独立した public crate として作成する。

### 設計原則

1. **Library-First**: CLI はオプショナル。ライブラリ API がプライマリインターフェース
2. **markdownlint 互換**: 公式 markdownlint の active rule ID との互換性を維持
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

### ルールカバレッジ

- 公式 markdownlint の全 active rule の check をサポート
- Fix 対応可否を全 active rule で metadata として明示
- `.markdownlint.json` / `.markdownlint.jsonc` の設定値を検証できる helper を提供

## Acceptance Criteria

- [ ] `cargo add katana-markdown-linter` でライブラリとして利用可能
- [ ] `lint()` / `fix()` のシンプルな公開 API
- [ ] docs.rs で 100% の API ドキュメンテーション
- [ ] markdownlint 互換の active rule ID
- [ ] 利用側アプリケーション固有の型や UI contract に依存しない
- [ ] `pulldown-cmark` ベースの AST パーサによる正確なルール評価
- [ ] CI（GitHub Actions）でのテスト・lint・ドキュメント生成
