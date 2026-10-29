# v0.20.0 品質再点検

対象commit: f0cbbcc084007758a4a70d8f423147e58996b3f0。今回の対象はread-only調査と再現、製品コード・設定・Issue・公開物の変更は含めない。

## 確定した改善候補

### P1: インラインescape意味論の共有

- MD011 raw delimiter scan: src/rules/markdown/rules/md011.rs:82。入力 `\(example\)[https://example.com]` が安全fixで `\[example\](https://example.com)` に変わる。escaped括弧はliteralであるため文字内容が変わる誤修正。
- MD049 delimiter scan: src/rules/markdown/rules/md049.rs:116。入力 `\*literal* and _valid_` でescaped openerを強調と誤認し、正しい `_valid_` を `*valid*` へ変更する。
- MD011例は3回fixしても誤った結果がbyte安定する。収束性だけでは正しさを保証できない。
- CommonMark0.31.2 §2.4: https://spec.commonmark.org/0.31.2/#backslash-escapes 。escaped punctuationは通常の文字として扱い、通常のMarkdown意味を持たない。
- 方針: escape/parity、強調・link境界をshared source adapterへ集約し、各ruleはpolicyと候補生成を担当する。既存code保護・公的range契約は維持。
- 再現: target/quality-followup/repro/results.json, stable-wrong-fix.json。

### P2: CLIとeditorの設定適用契約を統一

- src/cli/workflow/fmt.rs:42は設定を検証するが58でdefault formatterを呼び、実際の設定値を渡していない。LSPはsrc/lsp/server/config.rs:18でlayout_lint_options_fromへ渡す。
- 設定 {"MD007":{"indent":4},"MD041":false}、入力 `- parent\n    - child\n` はcheck exit0。fmtで子のindentを2へ変更、同じ設定のcheckがMD007/exit1になる。
- formatterのpolicyをlayout subsetへ絞ること自体は既存契約。問題はそのsubsetの設定値をCLI側だけ適用しないこと。
- 再現: target/quality-followup/repro/fmt-valid-results.json。

### P2: schema / validation / options conversionの契約差

- src/config/schema.rs:92はrule.enabled booleanを公開schemaへ含める。実CLI config schema出力も確認済み。
- src/rules/markdown/config/options.rs:67はenabledを解釈するが、config/validate.rs:76で全object keyを公式rule propertyとして検証しenabledをunknown propertyにする。
- {"default":false,"MD007":{"enabled":true,"indent":4}} をCLI checkへ渡すとconfig error/exit2。
- 方針: 共通の設定定義からschemaとvalidation/conversionを導出し、公開schemaで受理した設定を実CLIでも受理する契約テストを設ける。
- 再現: target/quality-followup/repro/config-results.json, schema-enabled.json。

### P2: 正解データmanifestのID一意性検証

- scripts/ci/document_answer_manifest.py:74はID重複を検出しない。document_answer_fix_runner.py:102はIDを一時ファイル名にする。
- 実manifestの先頭2件を同一IDにした複製でvalidate errors=[]、258 recordsに対し一時path257、先頭の内容が2件目に上書きされることを確認。
- 現在の実manifestは258 unique IDかつ258 unique input hash。この発見で現行258件の検査結果を無効とするものではない。
- 方針: runner起動前にID一意性・input/answer mapping整合を検査し、重複manifestの異常終了を契約化。
- 再現: target/quality-followup/repro/duplicate-result.json。

## 検証範囲の改善案（現行失敗とは区別）

- 258件すべてsource_repositoryがHiroyukiFuruno/katana-markdown-linter。200 publicの多くは同型の翻訳文書。入力内容の重複はないが、独立した構造・設定の多様性は件数ほど広くない。
- harnessは同一config、locale en、file check --fixに固定。全258件の直積増殖ではなく、少数の設定組合せ・file/stdin/library/editor入口・CRLF/Unicode/escape/入れ子の直交したケースを追加する。
- 意図的に入力を壊す変異テストと、style-only safe fixの表示上の意味保存を補助oracleで比較する。正解は現行出力から生成しない。
- 共通FixEngineのcycle/上限/unsafe除外/source slice適用には今回の限定監査で新しい確定不具合は見つからなかった。未確認の仮説をbug扱いしない。

## 実行結果

- rtk cargo build --locked --bin kml: exit0、native v0.20.0を直接実行。global kmlは0.16.1だったため再現には使用していない。
- 上記CLI再現、3回fix比較、manifest validation/copy衝突確認、config schema実出力確認を実施。
- scratchはtarget/quality-followup配下のみ。tracked差分なし、master同期済み、stash0、worktree1。
