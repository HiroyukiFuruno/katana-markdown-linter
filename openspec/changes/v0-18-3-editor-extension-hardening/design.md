# Editor Extension Hardening Design

## Context

`v0.18.1` は VS Code extension MVP、`v0.18.2` は Zed extension MVP を扱う。
どちらも thin wrapper として `kml lsp` を起動する。

MVP の後は、install docs、package metadata、release gate、post-release verification を揃え、
extension が「動く試作」ではなく「保守できる配布物」として扱える状態にする。

## Goals / Non-Goals

**Goals:**

- VS Code / Zed extension package check を release gate に固定する
- extension package と `kml` CLI の compatibility policy を定義する
- install / upgrade / troubleshooting docs を整える
- Marketplace / registry 公開に必要な metadata を準備する
- post-release verification で extension package と `kml lsp` launch を確認する
- Neovim は docs-only LSP configuration sample として維持する

**Non-Goals:**

- editor extension へ lint engine を移すこと
- editor ごとに異なる rule behavior を作ること
- automatic binary download を必須機能にすること
- Neovim plugin を repository に実装すること
- external marketplace への未検証 publish を自動実行すること

## Decisions

### D-1: extension version は CLI compatibility を明示する

extension package metadata は、対応する `kml` CLI version range を明示する。
MVP では `kml` を bundle しないため、extension 起動時に `kml --version` を確認し、
非対応 version の場合は明確な error を出す。

### D-2: release gate は package と launch の両方を見る

local と CI の gate は、extension package が build できることに加えて、
test fixture の `kml` binary path で `kml lsp` が initialize できることを確認する。

### D-3: marketplace publish は manual-ready に留める

この change では公開に必要な metadata、icon、README、license、package content を揃える。
Marketplace / registry への実 publish は、account / publisher 名 / review policy が確定してから
別 step として実施する。

### D-4: binary fallback は opt-in にする

extension は installed `kml` を使う方針を維持する。
npx / uvx fallback や automatic download は便利だが、起動遅延、network、security の判断が必要になる。
導入する場合は explicit opt-in の設定とし、default にはしない。

### D-5: editor docs は実際の操作別に整理する

docs は次を分ける。

- config schema validation
- Markdown diagnostics
- format / range format
- safe quick-fix
- binary path setting
- unsupported version troubleshooting
- Neovim docs-only sample

## Risks / Trade-offs

- marketplace account や publisher 名が未確定で release が止まる
  - this change は manual-ready metadata までを必須にし、publish 自体は確認済み条件に分ける
- extension が CLI version とずれて誤動作する
  - startup で `kml --version` を確認し、compatibility policy と一致させる
- package check が editor runtime に依存して CI で不安定になる
  - protocol smoke と package validation を分け、UI-dependent check は必須にしない
- fallback 経路を増やすと挙動が読みにくくなる
  - fallback は opt-in にし、default は explicit binary path / PATH のみにする

## Migration Plan

1. VS Code / Zed extension metadata を release-ready に整える
2. compatibility check を extension startup と tests に追加する
3. package validation target を Makefile と CI に固定する
4. release verification script に extension package / LSP launch check を追加する
5. README / editor docs / release runbook を更新する
6. Marketplace / registry publish 手順を manual-ready runbook として残す

## Open Questions

- VS Code publisher 名と Zed extension 公開先は implementation 開始時に確定する
- extension icon は KML icon を使うか、editor-specific variant を作るか確認する
