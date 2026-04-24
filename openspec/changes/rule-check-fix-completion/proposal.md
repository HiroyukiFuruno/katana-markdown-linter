## Why

phase2 で全active ruleの実装方針は進んだが、公式document由来のfixture matrixに基づく check / fix / config / edge の完了条件はまだ固定されていない。
`katana-markdown-linter` を組み込みライブラリとして信頼できる状態にするには、全ruleをfixture matrixに対して通す必要がある。

## What Changes

- `rule-fixture-parity-matrix` の成果物をDoRとして使う
- 全active ruleについて check behavior をfixtureで検証する
- fixable ruleについて safe fix behavior をfixtureで検証する
- config propertyについて valid / invalid をfixtureで検証する
- 境界値、イレギュラー、複合ruleの順序影響をテストへ追加する

## Impact

- rule実装の完了判断がfixtureベースになる
- fixの安全性が明示される
- config driftが利用者に露出する前に検出できる
