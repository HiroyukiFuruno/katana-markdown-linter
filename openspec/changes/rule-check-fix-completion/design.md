## Context

`mdxxx.rs` は check / fix の責務に閉じ、rule有効化、設定注入、順序制御は別責務とする設計方針で合意済みである。
このchangeはその責務境界を維持したまま、fixture matrixに基づいてrule behaviorを埋める。

## Goals / Non-Goals

**Goals:**

- 全active ruleのcheck fixtureを通す
- fixable ruleのfix fixtureを通す
- config valid / invalid fixtureを通す
- 境界値と異常系をrule単位で追加する
- rule間の順序依存がある場合はstrategy側の課題として可視化する

**Non-Goals:**

- CLI UXの大幅拡張はこのchangeでは扱わない
- upstream実装の丸写しはしない
- 安全性を確認できないfixを無理に実装しない

## Decisions

### 1. Fixture matrix を実装DoRにする

作業開始前に対象ruleのmatrixが存在していることを必須とする。
matrixが `manual_required` を含む場合、実装前に手動fixtureを追加する。

### 2. Rule file は check / fix だけを責務にする

`rules/markdown/rules/mdxxx.rs` はrule固有の検出とfix生成だけを持つ。
設定解釈、rule有効/無効、実行順序、複合fix制御は別moduleで扱う。

### 3. Fix は安全性優先

公式docまたは互換実装から安全な変換条件を確認できる場合だけfixを有効にする。
曖昧な場合はdiagnosticのみ出し、metadataでfix非対応理由を保持する。

## Risks / Trade-offs

- 全active ruleを一括で完了させると大きいため、rule id順の小taskで進捗を可視化する
- 複数ruleのfixが同じ範囲を編集する場合、strategy側の順序制御が必要になる
- 公式docに十分なexampleがないruleはmanual fixtureが必要になる

## Migration Plan

1. fixture matrixを読み込むtest harnessを作る
2. ruleごとのcheck fixtureを追加する
3. fixable ruleのfix fixtureを追加する
4. config fixtureを追加する
5. 境界値/異常系を追加する
6. rule別taskを完了にする
