# v0.18.6 Editor Publication Prerequisites

## 対象バージョン

- `v0.18.6`

## 目的

`v0.18.6` では、marketplace 公開導線の実行可否を「実装可」レベルで固定し、Neovim の対象外方針を明文化する。

## 変更内容

- VS Code / Zed の手動公開に必要な account / publisher / package 確認を固定する
- 公開開始前条件を満たさない場合は明示的に停止する
- Neovim は docs-only として明示し、実装実態に混ぜない
- その方針を `active-roadmap` と runbook で一貫化する

## 非対応範囲

- actual publish 実行自体は行わない
- エンジン本体変更は行わない
