## ADDED Requirements

### Requirement: repository SHALL contain a standalone Rust crate scaffold for the markdown linter

システムは、利用側アプリケーションから独立した Markdown linter を Rust crate として扱える scaffold を提供しなければならない（SHALL）。

#### Scenario: 初期 scaffold を作る

- **WHEN** phase1 が完了する
- **THEN** repository は `Cargo.toml` を持つ
- **THEN** repository は将来の Markdown rule 実装を受け入れられる source tree を持つ
- **THEN** repository は library-first の構造を持つ

### Requirement: local agent helper documents SHALL remain untracked

システムは、Codex / ClaudeCode / Antigravity 向けの local helper 文書を git 管理しないようにしなければならない（SHALL）。

#### Scenario: local helper 文書を生成する

- **WHEN** local helper として `skill.md`、`CLAUDE.md`、または `AGENTS.md` が作成される
- **THEN** git はそれを追跡しない
- **THEN** repository の tracked files には含まれない

### Requirement: project bootstrap SHALL provide a task entrypoint for future phases

システムは、future phase が使う project entrypoint を用意しなければならない（SHALL）。

#### Scenario: 次の phase に進む

- **WHEN** developer が phase2 以降の作業を開始する
- **THEN** `Markfile` から主要コマンドを起動できる
- **THEN** scaffold の変更が phase2 の rule 実装を妨げない
