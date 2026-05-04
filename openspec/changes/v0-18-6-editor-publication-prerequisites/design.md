# v0.18.6 Editor Publication Prerequisites Design

## 方針

`v0.18.6` では公開実行の前提条件を固定し、失敗時は中断する。

### D-1 account/publisher/package 条件の固定

public publish は、以下が明示されていることが前提。
- account
- publisher
- package name
- 権限（投稿可能）

### D-2 docs-only policy

Neovim は docs-only sample に限定し、plugin 実装は扱わない。

### D-3 文書と runbook の一致

`docs/editor-integration.md` の構成と runbook の前提条件は一致し、同一条件を runbook で示す。
