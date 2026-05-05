# Tasks

## 目標

- VS Code / Zed の editor integration を、公開前に最低限使える水準へ引き上げる。
- LSP の診断・安全な修正を CLI と同じ config 解決に従わせる。
- 最終 dogfood で誤診断・誤修正・設定反映漏れを潰してから、`v0.20.0` 公開 change へ渡す。

## 0. 事前確認

- [ ] 0.0 `jules-handoff.md` を読み、作業順・停止条件・証跡の残し方を確認する
- [ ] 0.1 `assess-editor-publication-readiness` の No-Go 理由を読み、公開作業をこの change に混ぜない
- [ ] 0.2 `src/cli/workflow/common.rs` の `load_effective_config_with_source` を読み、CLI の config 探索・読み込み責務を確認する
- [ ] 0.3 `src/lsp/document.rs` の `LintOptions::default()` 依存箇所を確認する
- [ ] 0.4 `src/lsp/server.rs` の document state と request handling を確認する
- [ ] 0.5 `tests/cli_lsp_contract.rs` を読み、LSP protocol test の追加方法を確認する
- [ ] 0.6 VS Code / Zed の現行 LSP 起動境界を確認し、rule 判定や config 探索を editor extension 側へ入れない方針を確認する

## 1. LSP config 解決

- [ ] 1.1 先に `tests/cli_lsp_contract.rs` に、`.markdownlint.json` で `MD018` を無効化した workspace の failing test を追加する
- [ ] 1.2 `load_effective_config_with_source` 相当の責務を `src/config/` から使える共有 API へ移す
- [ ] 1.3 CLI 側を共有 API 利用へ差し替え、既存 CLI behavior が変わらないことをテストで確認する
- [ ] 1.4 LSP diagnostics が document path から project config を解決するようにする
- [ ] 1.5 LSP code actions が diagnostics と同じ config 解決結果を使うようにする
- [ ] 1.6 config 未発見時だけ default config を使うようにする
- [ ] 1.7 invalid config で default config に黙って fallback しない test を追加する
- [ ] 1.8 config error を editor から確認できる diagnostic または response error として返す
- [ ] 1.9 config 変更通知時に開いている Markdown document を再診断する
- [ ] 1.10 `.markdownlint.json` と `.markdownlint.jsonc` の両方を test fixture で固定する

## 2. Editor diagnostics / actions

- [ ] 2.1 disabled rule の diagnostic が出ないことを LSP contract test で固定する
- [ ] 2.2 disabled rule の quick fix が出ないことを LSP contract test で固定する
- [ ] 2.3 rule option 変更が diagnostic に反映されることを LSP contract test で固定する
- [ ] 2.4 rule option 変更が quick fix に反映されることを LSP contract test で固定する
- [ ] 2.5 safe fix だけが editor quick fix として提示されることを確認する
- [ ] 2.6 unsafe fix が editor quick fix として提示されないことを確認する
- [ ] 2.7 formatting / range formatting が config error を黙って隠さないことをテストする

## 3. VS Code / Zed 検証

- [ ] 3.1 VS Code extension test に configured workspace ケースを追加できるか確認する
- [ ] 3.2 追加できる場合、VS Code test で `kml lsp` 起動と configured workspace の接続を確認する
- [ ] 3.3 追加できない場合、理由と代替の LSP contract test coverage を `docs/release-readiness/` に記録する
- [ ] 3.4 Zed extension 側で LSP 起動境界が共有 contract を壊していないことを `just zed-extension-check` または既存 check で確認する
- [ ] 3.5 editor extension check を release readiness evidence に紐づける

## 4. Final editor dogfood

- [ ] 4.1 `docs/release-readiness/v0.19.0-editor-capability-completion.md` を作成する
- [ ] 4.2 dogfood 対象 corpus と config 変更ケースを同ファイルに定義する
- [ ] 4.3 diagnostics / formatting / safe fixes / config changes を一連の evidence として記録する
- [ ] 4.4 finding を `false-positive` / `false-negative` / `bad-fix` / `config-gap` / `operation-gap` / `follow-up` に分類する
- [ ] 4.5 release-blocking finding が 0 件であることを確認する
- [ ] 4.6 `v0-20-0-editor-marketplace-publication` へ渡す evidence path と要約を残す

## 5. Review handoff

- [ ] 5.1 変更ファイル一覧を `docs/release-readiness/v0.19.0-editor-capability-completion.md` に記録する
- [ ] 5.2 追加/更新したテストと、そのテストが守る挙動を記録する
- [ ] 5.3 失敗から成功へ変わった代表 test を記録する
- [ ] 5.4 実行した command と結果を記録する
- [ ] 5.5 未実行の検証がある場合は理由と残リスクを記録する
- [ ] 5.6 reviewer が最初に確認すべき file / test / evidence path を記録する

## 6. 検証

- [ ] 6.1 `cargo test --test cli_lsp_contract --locked`
- [ ] 6.2 `cargo test --workspace --locked`
- [ ] 6.3 `just editor-extension-check`
- [ ] 6.4 `just dogfood`
- [ ] 6.5 `just ast-lint`
- [ ] 6.6 `scripts/openspec validate v0-19-0-editor-capability-completion --strict`

## Definition of Done

- [ ] D1 LSP diagnostics / quick fixes が project config を反映している
- [ ] D2 VS Code / Zed の editor-facing behavior が同じ LSP contract を使っている
- [ ] D3 final editor dogfood の release-blocking finding が 0 件である
- [ ] D4 Marketplace 公開作業を含めず、公開判断に必要な evidence だけを `v0.20.0` change へ渡せる
- [ ] D5 `v0-20-0-editor-marketplace-publication` の DoR が参照できる evidence path が存在する
