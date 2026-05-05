# Jules 作業指示

## 目的

`v0.19.0` では VS Code / Zed の拡張を公開しません。

この作業の目的は、editor 上で `kml` が実用できる状態にすることです。具体的には、Markdown ファイル上に project config を反映した診断を出し、そこから安全な修正を適用できる状態にします。

## 最初に読むもの

1. `openspec/changes/assess-editor-publication-readiness/`
2. `openspec/changes/v0-19-0-editor-capability-completion/proposal.md`
3. `openspec/changes/v0-19-0-editor-capability-completion/design.md`
4. `openspec/changes/v0-19-0-editor-capability-completion/specs/**/*.md`
5. `openspec/changes/v0-19-0-editor-capability-completion/tasks.md`

## 実装順

1. `tests/cli_lsp_contract.rs` に失敗するテストを追加する。
2. `src/cli/workflow/common.rs` の config 解決処理を確認する。
3. CLI と LSP で共有できる config 解決 API を `src/config/` 側へ移す。
4. `src/lsp/document.rs` の `LintOptions::default()` 依存をなくす。
5. `src/lsp/server.rs` で config 変更時に開いている document を再診断できるようにする。
6. VS Code / Zed は `kml lsp` 起動境界だけを確認する。rule 判定や config 探索を extension 側へ入れない。
7. 最後に実利用検証（dogfood）を行い、証跡を残す。

## 最初に追加するテスト

`tests/cli_lsp_contract.rs` に、以下のケースを追加してください。

- workspace に `.markdownlint.json` を置く。
- config で `MD018` を無効化する。
- Markdown 本文に `#Title` を入れる。
- `kml lsp` の `textDocument/publishDiagnostics` に `MD018` が出ないことを確認する。

このテストは、現状の `LintOptions::default()` 依存では失敗するはずです。先に失敗を確認してから実装してください。

## 必須テストケース

- `.markdownlint.json` の設定が LSP diagnostics に反映される。
- `.markdownlint.jsonc` の設定も同じように反映される。
- config がない場合だけ default config を使う。
- invalid config では default config に黙って戻らない。
- disabled rule の diagnostic は出ない。
- disabled rule の quick fix も出ない。
- safe fix は quick fix として出る。
- unsafe fix は quick fix として出ない。
- config 変更後、開いている Markdown document の diagnostics が再発行される。

## 触ってよい主な場所

- `src/config/**`
- `src/cli/workflow/common.rs`
- `src/lsp/**`
- `tests/cli_lsp_contract.rs`
- `editors/vscode/**` の test / 起動境界確認
- `editors/zed/**` の test / 起動境界確認
- `docs/release-readiness/v0.19.0-editor-capability-completion.md`

## 触らないもの

- VS Code Marketplace 公開処理
- Zed extension registry 公開処理
- `v0.20.0` 用の公開 workflow
- unsafe fix の editor quick fix 化
- editor extension 側への rule 実装
- editor extension 側への config 探索ロジック

## 証跡の残し方

`docs/release-readiness/v0.19.0-editor-capability-completion.md` を作成し、以下を記録してください。

- 変更したファイル一覧
- 追加/更新したテスト一覧
- 失敗から成功へ変わった代表テスト
- 実行した command と結果
- dogfood 対象 corpus
- config 変更ケース
- finding の分類
- 未実行の検証がある場合、その理由と残リスク
- reviewer が最初に見るべき file / test / evidence path

## 完了条件

- `cargo test --test cli_lsp_contract --locked` が通る。
- `cargo test --workspace --locked` が通る。
- `just editor-extension-check` が通る。
- `just dogfood` が通る。
- `just ast-lint` が通る。
- `scripts/openspec validate v0-19-0-editor-capability-completion --strict` が通る。
- `docs/release-readiness/v0.19.0-editor-capability-completion.md` に review 用の証跡が残っている。

## 停止条件

以下に該当する場合は、無理に進めず作業を止めて報告してください。

- config error を editor でどう表示するか判断できない。
- VS Code / Zed のどちらかで実操作検証ができない。
- safe fix と unsafe fix の区別が LSP 上で崩れる。
- release workflow や Marketplace 公開処理を触らないと進められそうに見える。
- dogfood で release-blocking finding が残る。
