## Definition of Ready
- [x] `proposal.md`、`design.md`、`specs` が揃っていること
- [x] 既存 Markdown rule 実装を seed として参照する範囲が確認されていること
- [x] ローカルのみの skill 文書の ignore pattern が `.gitignore` に記載されていること
- [x] crate name が `katana-markdown-linter`、library module name が `katana_markdown_linter`、初期 source directory が確定していること

## 1. Project Scaffold

- [x] 1.1 Markdown ルールのコピー先ディレクトリを作成する
- [x] 1.2 `Cargo.toml` の package / library skeleton を作成する
- [x] 1.3 `Markfile` を作成し、今後の check / test / format entrypoint を置く
- [x] 1.4 `.gitignore` を作成し、build artifact と local skill 文書を除外する

## 2. OpenSpec Bootstrap

- [x] 2.1 この repository 用の OpenSpec change scaffold を作成する
- [x] 2.2 Codex / ClaudeCode / Antigravity 向けの local skill 文書運用を定義する
- [x] 2.3 phase2 以降の change が参照する crate name、module layout、rule metadata location を文書化する

## Definition of Done
- [x] repository が独立 crate としての最小骨格を持つこと
- [x] local-only skill 文書が git 管理から外れていること
- [x] phase2 以降の change が参照する crate name、module layout、rule metadata location が決まっていること
