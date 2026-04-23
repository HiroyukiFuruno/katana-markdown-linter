# katana-markdown-linter 設計

## 概要

markdownlint 互換の Rust 製 Markdown リンターライブラリ。利用側アプリケーション固有の型や UI contract を知らない、独立クレートとして公開する。

## 1. クレート構成

```
katana-markdown-linter/
├── Cargo.toml
├── src/
│   ├── lib.rs              # 公開 API (lint, fix, LintOptions, etc.)
│   ├── parser/
│   │   └── mod.rs          # pulldown-cmark ベースの Markdown AST パーサ
│   ├── rules/
│   │   ├── mod.rs          # Rule trait 定義、ルールレジストリ
│   │   ├── heading.rs      # MD001-MD003, MD018-MD025, MD041
│   │   ├── list.rs         # MD004-MD007, MD029-MD032
│   │   ├── whitespace.rs   # MD009-MD012, MD047
│   │   ├── line.rs         # MD013
│   │   ├── code.rs         # MD014, MD031, MD038, MD040, MD046
│   │   ├── link.rs         # MD034, MD039, MD042, MD053-MD058
│   │   ├── emphasis.rs     # MD036, MD037, MD049-MD050
│   │   ├── blockquote.rs   # MD027-MD028
│   │   ├── html.rs         # MD033
│   │   └── structure.rs    # MD022, MD023, MD024, MD026, MD035, MD043-MD045, MD048, MD051-MD052
│   ├── fix/
│   │   └── mod.rs          # Fix エンジン（ルールごとの自動修正ロジック）
│   ├── config/
│   │   └── mod.rs          # .markdownlint.json 互換の設定読み込み
│   └── types.rs            # LintResult, FixResult, Severity, RuleMeta 等
├── tests/
│   ├── rules/              # ルールごとのユニットテスト
│   └── integration/        # 統合テスト
└── benches/                # パフォーマンスベンチマーク
```

## 2. 公開 API 設計

### Core Types

```rust
/// リント結果
pub struct LintResult {
    pub rule_id: String,         // e.g. "MD009"
    pub rule_name: String,       // e.g. "no-trailing-spaces"
    pub message: String,
    pub severity: Severity,
    pub line: usize,             // 1-indexed
    pub column: usize,           // 1-indexed
    pub end_line: usize,
    pub end_column: usize,
    pub fix: Option<Fix>,        // 自動修正情報（存在する場合）
}

/// 自動修正情報
pub struct Fix {
    pub range: Range,            // 置換範囲
    pub replacement: String,     // 置換テキスト
}

/// 修正結果
pub struct FixResult {
    pub content: String,         // 修正後のテキスト
    pub applied_fixes: usize,    // 適用された修正の数
}

/// ルール設定
pub struct LintOptions {
    pub rules: HashMap<String, RuleConfig>,  // ルールID → 設定
    pub default_severity: Severity,
}
```

### Core Functions

```rust
/// Markdown テキストをリントし、診断結果を返す
pub fn lint(content: &str, options: &LintOptions) -> Result<Vec<LintResult>>;

/// Markdown テキストのリント違反を自動修正する
pub fn fix(content: &str, options: &LintOptions) -> Result<FixResult>;

/// 利用可能なルール一覧を返す
pub fn available_rules() -> Vec<RuleMeta>;
```

## 3. Rule trait

```rust
pub trait Rule: Send + Sync {
    /// ルール ID (e.g. "MD009")
    fn id(&self) -> &str;

    /// ルール名 (e.g. "no-trailing-spaces")
    fn name(&self) -> &str;

    /// ルールの説明
    fn description(&self) -> &str;

    /// markdownlint 公式ドキュメント URL
    fn docs_url(&self) -> &str;

    /// このルールが自動修正をサポートするか
    fn is_fixable(&self) -> bool;

    /// テキストを評価し、違反を返す
    fn check(&self, content: &str, ast: &MarkdownAst) -> Vec<LintResult>;

    /// 違反を自動修正する（is_fixable() が true の場合のみ）
    fn fix(&self, content: &str, ast: &MarkdownAst) -> Option<String>;
}
```

## 4. Feature Flags

```toml
[features]
default = []
cli = []                            # `kml` CLI binary
jsonc = []                          # reserved for `.markdownlint.jsonc` support
```

## 5. Library Boundary

この crate は markdownlint-compatible な rule engine、configuration helper、fix engine、optional CLI を提供する。
利用側アプリケーションは、この crate の公開 API が返す汎用型を自分の domain model へ変換する責務を持つ。

Non-goals:

- editor UI / LSP UI の contract
- 特定アプリケーション専用の rule preset

## 6. 移行計画

### Phase 1: 基盤構築
- クレートの初期化（Cargo.toml, CI, docs.rs 設定）
- `Rule` trait と `lint()` / `fix()` API の定義
- `pulldown-cmark` ベースのパーサ基盤

### Phase 2: Rule Parity
- 公式 markdownlint の全 active rule の check 実装
- fixability metadata と安全な fix 実装
- 各ルールのユニットテスト作成

### Phase 3: Public Release 準備
- crates.io 向け metadata
- MIT license
- `cargo install` 可能な package 準備

### Phase 4: CLI
- `kml check`
- `kml fix`
- `kml init-config`
- `.markdownlint.json` / `.markdownlint.jsonc`
- `--format json`

### Phase 5: Upstream Update Tracking
- upstream default branch 追従
- rule document drift check
- config schema drift check
