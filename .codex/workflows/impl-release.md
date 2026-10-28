---
description: 指定バージョンのOpenSpec実装、品質確認、Draft release PR、merge後の自動Release、公開確認、事後整理までを一気通貫で進める Implementation & Release ワークフロー。
---

# /impl-release vX.Y.Z

指定バージョンの OpenSpec change に基づいて実装・検証し、release PR のレビュー、merge による Release workflow の起動、公開確認、事後整理まで進める。

## 前提

- 対象バージョンはユーザー入力から受け取る。例: `/impl-release v0.12.2`
- 対象 repository は `katana-markdown-linter`、default branch は `master`。
- 作業 branch は `release/*` のみを使う。version 実装には `release/vX.Y.Z` を使い、公開後整理で branch が必要な場合は既存 branch を再利用する。再利用できない場合のみ `release/cleanup-vX.Y.Z` を作成する。不要な branch を増やさず、`master` へ直接変更を加えない。追加 worktree は作らない。
- GitHub の `Release` workflow は `release/v*` から `master` への PR が merge されると自動起動する。PR merge 後に `rtk proxy just release` を重ねて dispatch しない。
- `rtk proxy just VERSION=vX.Y.Z release` は署名付き tag を push して `workflow_dispatch` から Release workflow を起動する手動 fallback。対応する自動 run が始まっていないことを確認した場合に限り使う。
- `rtk proxy just VERSION=vX.Y.Z release-verify` は公開後の tag、GitHub Release、crates.io 状態を検証する。
- OpenSpec の操作は必ず repository-local の `scripts/openspec` を経由する。bare `openspec` command は使わない。

## 停止条件

- OpenSpec の仕様にない不足が見つかり、判断を誤ると公開物・API・互換性・既存作業に大きな影響がある。
- 指定 version が公開済み release line から不自然に飛んでいる。例: 最新 stable が `v0.17.6` のとき `v0.18.7` を指定している場合は、`v0.17.7` または `v0.18.0` のどちらかを確認する。
- `KML_RELEASE_ALLOW_VERSION_LINE_OVERRIDE=1` は、修正 release などの理由をユーザーが明示承認した場合だけ使う。

それ以外では確認待ちで止めず、既存の安全ルールと gate に従って進める。Release workflow が失敗した場合は、失敗原因を調査・修正し、該当 workflow run を復旧する。失敗を避けるために `rtk proxy just release`、手動 tag、`rtk cargo publish` を使ってはならない。

## 参照するスキル

| 参照先 | 用途 |
| --- | --- |
| `.codex/skills/openspec-apply-change/SKILL.md` | OpenSpec tasks の実装 |
| `.codex/skills/openspec-sync-specs/SKILL.md` | 公開確認後の delta spec 同期 |
| `.codex/skills/openspec-archive-change/SKILL.md` | 公開確認後の完了 change の archive |
| `branch-hygiene` skill | merge・公開後の branch / 既存 worktree 整理 |
| `github:gh-address-comments` skill | PR review comment への対応 |

## Phase 0: 状態把握

1. `rtk git status --short --branch` を実行して差分、branch、未追跡ファイルを確認する。既存差分を依頼対象と混ぜない。
2. `rtk git fetch origin --tags` を実行する。
3. `Cargo.toml` の version、最新 tag、GitHub Release、crates.io の最新 version を確認する。
4. `rtk proxy just VERSION=vX.Y.Z release-target-check` を実行して、指定 version が自然な次版であることを確認する。失敗した場合は前提を見直し、正しい version が決まるまで停止する。
5. `openspec/changes/` から version に対応する active change を特定する。見つからない場合は `openspec/changes/active-roadmap.md` を確認する。
6. 対象 change の `proposal.md`、`design.md`、`tasks.md`、`specs/**/spec.md` と repository の `AGENTS.md` を読む。

## Phase 1: 実装

1. `release/vX.Y.Z` が既にあればそこで続ける。なければ `master` から作成する。例:

   ```bash
   rtk git switch master
   rtk git pull --ff-only origin master
   rtk git switch -c release/vX.Y.Z
   ```

2. 対象 change の Definition of Ready と tasks を確認し、合意済みの範囲を実装する。task の進捗は完了時に `tasks.md` へ反映する。
3. 公開前に全 task の archive を強制しない。change は公開確認後の spec 同期・archive まで active のまま維持する。
4. 実装上の判断材料が不足する場合は、独立して進められる作業を続けつつ、必要事項だけ確認する。

## Phase 2: 品質確認

基本 gate はすべて実行する。

```bash
rtk proxy just fmt-check
rtk proxy just lint
rtk proxy just ast-lint
rtk cargo test --workspace --locked
rtk proxy just dogfood
rtk git diff --check
```

release 前にも既存の release gate をすべて実行し、検証範囲や基準を下げない。

```bash
rtk proxy just VERSION=vX.Y.Z release-target-check
rtk proxy just VERSION=vX.Y.Z release-check
```

対象 OpenSpec change が追加 verification を定めている場合は、それも実行する。

## Phase 3: Draft release PR とレビュー

1. Cargo、README、public docs、CHANGELOG、wrapper、editor、Zed の version metadata を対象版に同期する。public Markdown は英語で書き、`README.md` または `docs/**` を変更したら `rtk proxy just ast-lint` を実行する。
2. OpenSpec change はこの段階では archive しない。実装・検証結果を active change と `tasks.md` に記録する。
3. commit は署名付きで作成し、push 前に `rtk git status --short --branch` と staged diff を確認する。branch を push して、GitHub 上の head commit が `verified=true` / `reason=valid` であることを確認する。

   ```bash
   rtk git push -u origin release/vX.Y.Z
   ```
4. `release/vX.Y.Z` から `master` を base にした Draft PR を作成する。

   ```bash
   rtk gh pr create --draft --title "Prepare vX.Y.Z release" --base master --head release/vX.Y.Z --body-file <pr-body-file>
   ```

5. Draft のまま `@codex review` と自己レビューを実行し、レビュー結果・CI comment・review thread をすべて取得する。
6. 指摘を優先度と根拠で判断する。P0/P1 は必ず修正する。P2/P3 も採否を判断し、修正しない指摘には根拠を返信して thread を resolve する。
7. 未対応 P0/P1 が0件であることを再確認し、すべての review thread への対応・返信・resolve が終わってから Draft を解除する。
8. `Ready for review` に昇格し、required checks がすべて pass し、全 commit の GitHub verified signature を確認してから merge する。`--admin` は使わない。

   ```bash
   rtk gh pr ready <PR番号またはURL>
   ```
9. `master` へ merge し、PR が merge 済みであることを確認する。

   ```bash
   rtk gh pr merge --merge --delete-branch <PR番号またはURL>
   ```

推奨 PR body:

```markdown
## Summary
- Prepare vX.Y.Z release
- Implement <change-id>

## Verification
- rtk proxy just VERSION=vX.Y.Z release-check
```

## Phase 4: merge による自動 Release

1. `release/vX.Y.Z` から `master` への PR merge によって Release workflow が自動起動する。merge 後、clean な `master` を fast-forward してから次で run を確認・監視する。

   ```bash
   rtk git switch master
   rtk git pull --ff-only origin master
   rtk proxy just release-status
   rtk gh run list --repo HiroyukiFuruno/katana-markdown-linter --workflow Release --limit 5
   ```

2. 対応する自動 Release run が存在する場合は、その run を監視する。queued、running、failed、cancelled のどの状態でも `rtk proxy just release` を追加実行しない。
3. PR merge 後に自動 run が一度も開始していないことを GitHub Actions 上で確認できた場合に限り、手動 fallback として一度だけ実行する。

   ```bash
   rtk proxy just VERSION=vX.Y.Z release
   ```

   自動 run の存在が曖昧な場合は dispatch せず、先に状態を確認する。workflow 失敗時の回避策や二重 dispatch として `rtk proxy just release` を使ってはならない。

## Phase 5: 公開後の検証・OpenSpec 完了・整理

1. Release workflow の成功後、全テスト実行後の release-target state を対象 version で再生成する。負例テストは `target/release-verify-state.json` を fixture version で上書きするため、公開 verification の直前に必ず実行する。

   ```bash
   rtk proxy just VERSION=vX.Y.Z release-target-check
   rtk proxy just VERSION=vX.Y.Z release-verify
   ```

2. GitHub Release、crates.io、対象 wrapper/editor の公開結果を確認し、実際の URL・version・CI run ID を証跡に残す。公開未完了や検証失敗を完了として記録しない。
3. 公開後に残作業を完了し、OpenSpec `tasks.md` を実態に合わせて更新する。次の ledger gate を通してから archive する。

   ```bash
   rtk proxy just VERSION=vX.Y.Z release-task-ledger-check
   ```

4. OpenSpec delta spec を main spec と照合して同期し、完了 change を archive する。必ず `openspec-sync-specs` / `openspec-archive-change` の手順を使い、CLI 操作は `scripts/openspec` 経由にする。
5. `branch-hygiene` skill に従い、merge・公開済みの `release/vX.Y.Z`、不要な既存 worktree、不要な local refs を確認して整理する。未merge・未commit の作業は削除しない。新しい worktree は作らない。
6. 完了報告には GitHub Release URL、公開 package version、release workflow run、release verification、spec sync/archive、整理した branch / worktree を記録する。次版に回した課題は roadmap または OpenSpec change に残す。

## 完了条件

- [ ] 対象 version が release target check を通過している
- [ ] 基本 gate、OpenSpec verification、`release-check` が成功している
- [ ] Draft PR 上で `@codex review` と自己レビューを行い、指摘を対応・返信・resolve している
- [ ] P0/P1 が0件で、required checks と commit signature が確認済み
- [ ] PR が `master` に merge されている
- [ ] merge 起動の Release workflow が成功している。workflow が自動起動しなかった場合だけ手動 fallback が一度実行されている
- [ ] `release-verify` と公開 artifact 確認が成功している
- [ ] 公開後に OpenSpec ledger を完了し、delta specs を同期して change を archive している
- [ ] `branch-hygiene` に沿った整理と次版課題の記録が完了している
