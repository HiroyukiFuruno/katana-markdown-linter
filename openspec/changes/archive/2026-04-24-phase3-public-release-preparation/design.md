## Context

Rust の publish は、単にビルドが通るだけでは足りない。
`Cargo.toml` の metadata、README、license、package include / exclude、そして `cargo publish` 前の dry-run が必要になる。

## Goals / Non-Goals

**Goals:**

- crates.io への publish 条件を満たす
- `cargo install` で binary を導入できる package にする
- release 手順を自動検証できるようにする
- 公開ページに必要な README / metadata を揃える

**Non-Goals:**

- CLI の最終 UX
- ルール実装の追加
- editor plug-in の配布

## Decisions

### 1. publish 前提の metadata を Cargo.toml に集約する

`license`、`readme`、`repository`、`description`、`keywords`、`categories` の公開情報は package metadata に明示する。
Cargo Book が要求する publish 条件に合わせるためである。

### 2. publish 前に dry-run を必須にする

`cargo publish` は永久的な操作なので、`cargo publish --dry-run` か `cargo package` を先に通す。
パッケージ内容と manifest の不整合を早期に発見できる。

### 3. binary target を install 可能な形で維持する

`cargo install` が扱えるのは binary target であるため、ライブラリだけに閉じない。
将来の CLI を install 可能な package として見せるための前提になる。

## Risks / Trade-offs

- publish metadata を早く固定しすぎると、未完成の CLI 名称や機能セットを変えにくくなる
- package include / exclude を誤ると、公開 tarball に不要物が入る
- binary target と library API の両立には、ディレクトリ整理が必要になる

## Confirmed Decisions

以下はユーザー確認済みの決定事項である。

- crates.io package name は `katana-markdown-linter` とする
- executable name は `kml` とする
- license は MIT とする
- README は general-purpose markdownlint-compatible crate として記述し、特定アプリケーション専用の位置付けを持たせない

## User Decisions

以下は publish 準備に入る前にユーザーと協議して確定する。

- initial publish を public crates.io に出すか、Git dependency 運用を先に挟むか

## Migration Plan

1. package metadata と公開ポリシーを `Cargo.toml` / README / LICENSE で確定する
2. dry-run / package validation を CI に入れる
3. README に library API、CLI install、configuration、release policy を記載する
4. phase4 の CLI を publish-ready package に接続する
