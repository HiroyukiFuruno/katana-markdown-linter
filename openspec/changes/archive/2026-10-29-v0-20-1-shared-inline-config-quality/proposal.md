## Why

v0.20.0の追加監査で、escaped delimiterの誤修正、CLI fmtとeditorの設定適用差、schema/validation間の契約差、検証manifest重複IDによる上書きを再現した。正解への一致と3回fixの収束を維持しつつ、同じ不整合を別の入口で再実装しない境界へ改善する。

## What Changes

- inline escapeと強調delimiterのsource解析を共有し、MD011/MD049と関連する強調ruleが同じ原文境界を使う。
- formatterのlayout設定をCLI file/stdinとeditorで共有し、共通rule設定のschema/validation/options変換を整合する。
- manifestのIDとinput/answer mappingを起動前検証し、一時ファイル衝突を拒否する。
- 独立した正解データと少数の設定・入口・文字/改行/入れ子パターン行列を追加し、意味保存と3回収束を検証する。
- 依存監査、全既存品質gate、Draftレビュー、v0.20.1公開確認、事後整理まで実施する。

## Capabilities

### New Capabilities

- `shared-inline-config-contracts`: escaped inline構文の共有、設定schema/validation/formatter適用の一貫性。

### Modified Capabilities

- `document-answer-fix-evaluation`: manifest一意性と構造/設定行列、正解一致・意味保存の検証。

## Impact

対象はこのrepoのみ。Markdown source adapter・MD011/MD049/関連強調rule、config/formatter入口、検証スクリプト・fixturesに限定する。公開APIとbyte/UTF-16境界を維持し、品質baseline/thresholdを下げない。Marketplace新規公開は既存v0.21.0計画で保持する。
