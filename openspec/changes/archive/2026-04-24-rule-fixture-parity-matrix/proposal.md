## Why

`katana-markdown-linter` は markdownlint 互換を目指しているが、現状の unit test は公式 rule document に記載されたパターンを網羅していない。
rule ごとの check / fix / config behavior を拡充する前に、公式 document 由来の fixture matrix を作り、実装漏れ・fix漏れ・設定漏れを機械的に可視化する必要がある。

## What Changes

- `DavidAnson/markdownlint` default branch の `doc/md*.md` を解析し、rule fixture source として扱う
- 各 rule ごとに `check_pass` / `check_fail` / `fix` / `config_valid` / `config_invalid` / `edge` の matrix を作る
- 公式 document に明示された examples / parameters / aliases / fixability を fixture metadata に取り込む
- 現時点で自動fixture化できないパターンは `manual_required` として明示する
- matrix を JSON / Markdown summary として出力し、後続実装changeの進捗表として使えるようにする

## Impact

- rule実装の完了条件が「感覚」ではなくfixture matrixに基づく
- 公式doc変更時にテスト更新漏れを検出できる
- phase2のrule parityを実装品質・テスト品質の両面から再評価できる
