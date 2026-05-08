# Design

## 方針

`v0.19.0` 系は、Marketplace 公開ではなく editor 機能の実用化を完了するための開発ラインとする。完了条件は「動いて見える」ではなく、config 反映、safe fix、整形、最終 dogfood の evidence が揃うこと。

## 実装者向け前提

- 最初に `assess-editor-publication-readiness` を読み、現状の No-Go 理由を確認する。
- 実装順は `LSP config 解決 -> LSP contract test -> VS Code / Zed 境界確認 -> final dogfood` とする。
- 先に failing test を追加し、`LintOptions::default()` に依存している現状で落ちることを確認してから実装する。
- Marketplace 公開、release workflow の publish 実行、`v0.20.0` の公開作業はこの change では触らない。
- VS Code / Zed extension 側に rule 判定や config 探索を重複実装しない。editor extension は `kml lsp` 起動境界に留める。
- 実装結果は別レビューされる前提で、各 task の完了時に「変更ファイル」「追加/更新したテスト」「確認した command」を `docs/release-readiness/v0.19.0-editor-capability-completion.md` に残す。

## LSP config 解決

- 現在の CLI config 探索は `src/cli/workflow/common.rs` の `load_effective_config_with_source` にある。
- この関数は `pub(super)` で CLI 内に閉じているため、共有するなら `src/config/` 側へ移す。
- 共有 API は `MarkdownLintConfig` と config source path を返し、CLI と LSP の両方から呼べる形にする。
- LSP は document URI から file path を解決し、workspace 内の `.markdownlint.json` / `.markdownlint.jsonc` を探索する。
- config が見つからない場合は CLI と同じ default config を使う。
- config error は黙って握りつぶさず、editor diagnostic または明確な LSP error として表示できる形にする。
- config file の変更通知を受けた場合、開いている Markdown document の診断を再計算する。

## 既存の問題箇所

- `src/lsp/document.rs` の `diagnostics` と `code_actions` は `LintOptions::default()` を直接使っている。
- `src/lsp/server.rs` は document content だけを保持しており、config source や workspace state を持っていない。
- `tests/cli_lsp_contract.rs` は LSP の基本動作だけを見ており、configured workspace の回帰テストがない。
- `editors/vscode/src/extension.ts` と `editors/zed/src/lib.rs` は `kml lsp` 起動境界なので、原則として config 解決の修正対象ではない。

## Editor actions

- 診断は config で有効な rule と option だけを反映する。
- code action は safe fix のみを提示する。
- unsafe fix は editor 上の quick fix として提示しない。
- formatting / range formatting は config error がある状態で誤った成功に見せない。
- VS Code / Zed 固有コードは LSP 起動と設定UIに留め、rule 実装や config 解決を editor extension 側へ重複させない。

## Final dogfood

- 最終 dogfood は、専用 fixture だけでなく実リポジトリ相当の Markdown corpus で実施する。
- 診断、整形、安全な修正、config 変更時の再診断を同じ evidence にまとめる。
- 誤診断、誤修正、設定反映漏れ、未分類 finding が 1 件でも残る場合は `v0.20.0` 公開へ進めない。
- evidence は `docs/release-readiness/` 配下に残し、`v0-20-0-editor-marketplace-publication` が参照できる名前にする。

## 非対象

- VS Code Marketplace 公開。
- Zed extension registry 公開。
- 公開用 release job の実行。
- unsafe fix の editor quick fix 化。

## 検証

- Rust LSP protocol test で config 反映済み diagnostics / code action を固定する。
- VS Code extension test で configured workspace の起動と LSP 接続を確認する。
- Zed extension test / static check で `kml lsp` 起動境界を確認する。
- `just editor-extension-check` と最終 editor dogfood を release readiness evidence に紐づける。

## Review handoff

- Reviewer は `docs/release-readiness/v0.19.0-editor-capability-completion.md` を入口にして結果を確認する。
- Handoff evidence には、変更ファイル一覧、テスト一覧、失敗から成功へ変わった test、未対応 finding、`v0.20.0` へ渡す evidence path を含める。
- 実装者は「通した command」だけでなく、config 反映の代表ケースとして使った fixture / workspace path を記録する。
- 未実行の検証がある場合は、理由と残リスクを同ファイルに書く。
- Reviewer が確認しやすいよう、PR / commit message だけに証跡を閉じ込めない。

## 最低限のテストケース

- `.markdownlint.json` で `MD018` を無効にした workspace では、`#Title` の `MD018` diagnostic が出ない。
- `.markdownlint.jsonc` でも同じ config 解決が動く。
- config が壊れている workspace では default config に黙って戻らず、config error を確認できる。
- config 変更後、開いている document の diagnostics が再発行される。
- disabled rule の quick fix は提示されない。
- safe fix は提示されるが、unsafe fix は提示されない。
