# v0.18.5 Release Verification Hardening Design

## 方針

`v0.18.5` は `release-check` と `release-verify` の解釈差を許さない。

### D-0 共通 state 仕様（v0.18.4 と同一）

- `release-verify` と `release-check` は同一の `state` 参照先を使う。
- state ファイル: `target/release-verify-state.json`（JSON）
- 必須キー:
  - `version`
  - `editor_artifacts.vscode.state`
  - `editor_artifacts.zed.state`
  - `publish_blockers`
  - `release_decision`
- `release_decision` は `publish_blockers` が空で `editor_artifacts.*.state` が許容値なら `allow_release`。

### D-1 partial publish は停止条件にする

GitHub Release だけ先行し、他の公開経路が未完了なケースは停止とする。

### D-2 停止条件の機械化

- external registry 検証失敗
- wrapper 検証失敗
- Homebrew tap / formula 更新停止
- marketplace publish 前提未満足

### D-3 one source of truth

上記条件の評価は共通 script と同一チェックキーから取る。

### D-4 v0.18.5 における制約

v0.18.5 では共通 state ファイルの骨格を実装し、版管理（version）と基本的な editor 状態の検証を先行させる。
external registry / wrapper / Homebrew のゲート失敗を自動的に state.publish_blockers に集約する完全実装は v0.18.6 以降の課題とする。
