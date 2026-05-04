# v0.18.4 Editor Release Gate Finalization Design

## 背景

`release` 系は `published` と `deferred` の解釈が曖昧だと `v0.19.0` 判定にノイズが残る。
本 change ではその曖昧性を排除する。

## 方針

- `release-verify` は editor artifact の状態を機械的に `published` / `deferred` として出す。
- `release-check` はその状態の意味を前提にし、runbook に基づく一貫した判定をする。
- manual publish が必要な経路は本 change では開始しない。

## 判定

- `published`: artifact が公開済みで、再公開と矛盾しない場合。
- `deferred`: 公開未完了、部分公開、条件不備などで公開側が未達の場合。

### 判定対象キー（次の 3 要素を固定）

- `editor_artifacts.vscode.state`
- `editor_artifacts.zed.state`
- `publish_blockers`

## リスクと対策

- 監査不能な状態遷移
  - 監査ログ形式を固定し、`release-check` と紐付ける。
- runbook と実装の齟齬
  - `active-roadmap` と spec の用語を同一化する。

## 出力形式（v0.18.4）

- `scripts/release` が保存する state ファイル（例: `target/release-verify-state.json`）は、少なくとも次を必須とする。
  - `version`: `v0.18.4`（判定対象）
  - `editor_artifacts.vscode.state`: `published|deferred`
  - `editor_artifacts.zed.state`: `published|deferred`
  - `publish_blockers`: `[]` または blocker ID 配列
  - `release_decision`: `allow_release|stop_release`
