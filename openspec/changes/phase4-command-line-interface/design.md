## Context

公式 markdownlint は library と CLI の両方を持ち、CLI は file 入力、config 読み込み、fix を扱う。
この phase では、Rust 実装としての見通しを優先しつつ、ユーザーが期待する操作を最小の command set で実現する。

## Goals / Non-Goals

**Goals:**

- `check` と `fix` を明確に分ける
- `.markdownlint.json` の初期作成を 1 コマンドで行えるようにする
- config discovery を実用的な順序で実装する
- 失敗時の exit code を自動化しやすくする

**Non-Goals:**

- VS Code などの editor integration
- remote service 連携
- rule 実装の追加

## Decisions

### 1. CLI は subcommand ベースにする

`check`、`fix`、`init-config` を subcommand として分ける。
用途が明確になり、後からオプションを増やしても混ざりにくい。

### 2. config discovery は `.markdownlint.json` と `.markdownlint.jsonc` を第一級にする

phase4 で読み込む config は `.markdownlint.json`、`.markdownlint.jsonc`、および `--config <path>` で指定された JSON / JSONC file に限定する。
YAML / TOML など他形式の読み込みは、この phase の scope から外す。

### 3. check と fix は exit code を分ける

`check` は違反があれば非 zero、`fix` は修正後の再 lint まで見て最終状態を判断する。
automation や CI で扱いやすくするためである。

## Risks / Trade-offs

- CLI オプションを増やしすぎると phase4 の範囲が膨らむ
- config discovery の precedence は `--config`、current directory の `.markdownlint.json`、current directory の `.markdownlint.jsonc`、parent directory search の順に固定する
- check/fix の出力形式を曖昧にすると、将来の integration が壊れやすい

## Confirmed Decisions

以下はユーザー確認済みの決定事項である。

- executable name は `kml` とする
- config support は `.markdownlint.json` と `.markdownlint.jsonc` を phase4 から含める
- output format は human-readable text に加えて `--format json` を phase4 から含める

## User Decisions

以下は CLI 実装前にユーザーと協議して確定する。

- `fix` command が一部違反を残した場合の exit code を non-zero にするか
- glob の展開を shell に任せるか、CLI 内で glob pattern を解釈するか

## Migration Plan

1. command set を最小限で固定する
2. config discovery を `.markdownlint.json` / `.markdownlint.jsonc` に限定して実装し、helper の出力は `.markdownlint.json` に固定する
3. exit code と file input の contract を固める
4. library API との分離を確認する
