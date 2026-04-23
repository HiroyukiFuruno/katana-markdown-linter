## Why

phase1 の scaffold 上で、`markdownlint` 本家の文書に準じた rule 実装を積み上げる必要がある。
この phase の目的は、`check`、`fix`、`config` の内部品質を上げ、後続の公開準備と CLI 化に耐える rule engine を作ることだ。

## What Changes

- 公式 `markdownlint` documentation に基づく rule catalog を整備する
- 公式 documentation に準じた check 実行を全 rule に対して提供する
- upstream implementation または official documentation から安全な fix behavior を確認できる rule は自動修正可能にし、修正不能な rule は理由付きで明示的に分離する
- `.markdownlint.json` の作成・読み込み・検証を行う helper を整備する

## Impact

- Markdown lint engine の振る舞いが利用側アプリケーションから独立した contract として明確になる
- phase3 で公開する crate の品質境界が、rule coverage / fix coverage / config coverage の表で説明できる
- phase4 の CLI は、この rule engine をそのまま呼び出せるようになる
