## Why

既存実装を seed として参照しつつ、Markdown linter を利用側アプリケーションから独立した共通 library crate として成立させる最初の土台が必要である。
この phase では、以後の rule 移植や公開準備に先立って、cargo で扱える独立 crate の骨格と、OpenSpec による作業前提を整える。

## What Changes

- 既存 Markdown ルール群を seed として取り込めるディレクトリ構成を作る
- `Cargo.toml`、`.gitignore`、`Markfile` を含む初期プロジェクト設定を用意する
- `openspec init` で生成される構成を確認し、Codex / ClaudeCode / Antigravity 向けの skill 文書は git 管理外に置く
- 今後の phase で差し替えるための最小限の library / binary entrypoint を用意する

## Impact

- 以後の phase2 以降で、rule 実装と config/fix の差分をこの repository 内だけで追える
- 既存の `katana` 側実装へ依存する期間を短くできる
- `cargo install` で使える最終形に向けて、package レイアウトを先に安定させられる
