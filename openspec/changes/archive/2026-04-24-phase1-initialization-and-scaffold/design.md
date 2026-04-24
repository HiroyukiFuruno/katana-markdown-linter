## Context

この repository は、Markdown linter を利用側アプリケーションから独立して配布するための共通 library crate である。
phase1 では、まだ rule の完全移植は行わず、後続 phase が壊れにくい構造だけを先に固定する。

## Goals / Non-Goals

**Goals:**

- 独立した Rust package としてビルドできる最小構成を作る
- Markdown ルール群のソースを移し替えやすいディレクトリ構成を作る
- OpenSpec change を継続運用できる状態にする
- `skill.md` 系のローカル補助ファイルを git 管理外に逃がす

**Non-Goals:**

- ルールの完全実装
- check / fix の完全互換
- crates.io 公開
- CLI の最終 UX

## Decisions

### 1. package は library-first にする

最終的に cargo install 可能な binary を持つとしても、phase1 では library crate を中心に骨格を作る。
後続 phase で binary target を追加しても、公開 API を壊しにくいためである。

### 2. Markdown ルールの実装先は repository 内に固定する

既存実装を seed として参照する場合も、取り込み先はこの crate の Markdown rule module 名前空間に固定する。
移行期の diff を追いやすくするためである。

### 3. ローカル skill 文書は追跡しない

`Codex` / `ClaudeCode` / `Antigravity` 向けの補助文書は、生成・再生成される前提にして gitignore へ入れる。
OpenSpec の change 本体だけをソース・オブ・トゥルースにする。

## Risks / Trade-offs

- 初期 scaffold を薄くしすぎると phase2 で設計のやり直しが増える
- 逆に skeleton を詰め込みすぎると、まだ固まっていない CLI / publish 方針を早く固定しすぎる

## Confirmed Decisions

以下はユーザー確認済みの決定事項である。

- crate name と library module name は `katana-markdown-linter` / `katana_markdown_linter` で固定する
- phase1 時点では binary target を置かず、phase4 で `kml` executable として追加する
- local helper documents は `skill.md`、`CLAUDE.md`、`AGENTS.md`、`*.skill.md` を ignore 対象にする

## Migration Plan

1. ルート構成と package metadata を確定する
2. 既存 Markdown ルールの移植先モジュールを作る
3. local-only helper files の ignore 方針を固定する
4. phase2 以降の implementation へ引き継げる task 分割を作る
