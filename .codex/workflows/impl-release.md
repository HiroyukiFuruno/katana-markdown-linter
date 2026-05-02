---
description: 指定バージョンのOpenSpec実装、品質確認、リリース準備、PR作成、GitHub Release、crates.io publish、自己レビュー、事後整理までを自律的に遂行する Implementation & Release Autopilot ワークフロー。
---

# /impl-release vX.Y.Z

指定バージョンの OpenSpec change に基づく実装・修正から、リリース準備、PR 作成、GitHub Release、crates.io publish、release verification、branch hygiene までを一気通貫で進める。

## 前提

- 対象バージョンはユーザー入力から受け取る。例: `/impl-release v0.12.2`
- 作業対象 repository は `katana-markdown-linter`
- default branch は `main`
- `main` は GitHub branch protection で signed commits を必須にし、admin にも保護を適用する
- release は `just VERSION=vX.Y.Z release` を正とする
- publish 後 verification は `just VERSION=vX.Y.Z release-verify` を正とする
- `just VERSION=vX.Y.Z release` が失敗した場合、手動 tag 作成や `cargo publish` 直叩きで迂回しない

## 停止ルール

この workflow は release 完了まで自律的に進める。次の条件に当たる場合だけ作業を中断し、ユーザーの判断を仰ぐ。

- OpenSpec tasks に記載がない不足や想定外が進行中に露見した
- その不足を Codex の判断だけで補うと、公開物、API、互換性、release 成果物、または既存作業を壊すリスクが高い
- 指定 version が公開済み release line から見て不自然に飛んでいる
  - 例: 最新 stable が `v0.17.6` なのに `v0.18.7` を指定している
  - この場合は `v0.17.7` の patch 継続か、`v0.18.0` の minor 開始かをユーザーに確認する

それ以外では「進めてもよいか」の確認で止めない。commit、push、PR 作成、CI 失敗の修正、merge、release、release verification、branch hygiene は、既存の安全ルールと検証結果に従って継続する。

## Branch Naming

`impl-release` では branch 名を次に統一する。

- 統合ブランチ: `release/vX.Y.Z`
- 補助ブランチが必要な場合: `feature/vX.Y.Z-<short-slug>`

この workflow 内では `fix/vX.Y.Z-*`、`chore/vX.Y.Z-*`、`release/vX-Y-Z` は使わない。
既存 branch が別命名の場合は、未 push なら rename し、push 済みなら新しい統一 branch を作って移す。

## 参照するスキル

| 参照先 | 用途 |
| --- | --- |
| `.codex/skills/openspec-apply-change/SKILL.md` | OpenSpec tasks の実装 |
| `.codex/skills/openspec-archive-change/SKILL.md` | 完了 change の archive |
| `branch-hygiene` skill | PR merge 後、release 後の branch / worktree 整理 |
| `github:gh-address-comments` skill | PR review comment 対応が必要な場合 |

## Phase 0: 状態把握

1. `git status --short --branch` を実行し、既存差分を確認する。
2. `git fetch origin --prune --tags` を実行する。
3. `Cargo.toml` の現在 version、最新 tag、最新 GitHub Release、crates.io 最新 version を確認する。
4. `just VERSION=vX.Y.Z release-target-check` を実行し、指定 version が公開済み release line の自然な次版であることを確認する。
   - 失敗した場合は、ユーザー指定を正しい前提にせず停止する。
   - `KML_RELEASE_ALLOW_VERSION_LINE_OVERRIDE=1` は、修正リリースなどの理由をユーザーが明示承認した場合だけ使う。
5. 対象 version の active OpenSpec change を特定する。
   - 例: `v0.12.2` -> `openspec/changes/v0-12-2-*`
   - 見つからない場合は `openspec/changes/active-roadmap.md` を確認する。
6. 対象 change の `proposal.md`、`design.md`、`tasks.md`、`specs/**/spec.md` を読む。

## Phase 1: 実装

1. 対象 version 用の作業ブランチを用意する。
   - 既存の対象ブランチがある場合は継続する。
   - 無い場合は `main` から `release/vX.Y.Z` を作成する。
   - 原則として task ごとに branch は切らない。
   - 並列作業や大きな分離が必要な場合だけ、`release/vX.Y.Z` から `feature/vX.Y.Z-<short-slug>` を作成する。
2. `tasks.md` の Definition of Ready を上から確認し、満たしていない項目は先に解消する。
3. 未完了 task を上から順に実装する。
4. 進捗は task 完了直後に `tasks.md` の checkbox へ反映する。
5. 判断材料が不足している場合だけユーザーに確認する。実装可能な範囲で止まらない。

## Phase 2: 品質確認

基本 gate:

```bash
just fmt-check
just lint
just ast-lint
cargo test --workspace --locked
just dogfood
git diff --check
```

release 前 gate:

```bash
just VERSION=vX.Y.Z release-target-check
just VERSION=vX.Y.Z release-check
```

対象 change の `tasks.md` に追加 verification がある場合は、それも実行する。

## Phase 3: リリース準備 PR

1. `Cargo.toml` / `Cargo.lock` / README / docs / CHANGELOG / OpenSpec archive を対象 version に合わせる。
2. public docs は英語で書く。`README.md` または `docs/**` を変更した場合は `just ast-lint` を実行する。
3. 対象 OpenSpec change の全 task が完了したら、`openspec-archive-change` skill に従って archive する。
4. commit 前に `git status --short --branch` と `git diff --cached --stat` を確認する。
5. commit 作成後、GitHub 上の head commit が `verified=true` / `reason=valid` であることを確認する。
6. release PR を次の形式で作成し、CI を監視する。

```bash
gh pr create --title "Prepare vX.Y.Z release" --base main --body-file <pr-body-file>
```

`gh pr create` には source branch 削除 option がない。PR merge 時に必ず `gh pr merge --merge --delete-branch` を使い、merge 後に local / remote branch が削除されたことを `branch-hygiene` skill で確認する。

推奨 PR body:

```markdown
## Summary
- Prepare vX.Y.Z release
- Complete <change-id>
- Archive completed OpenSpec change

## Verification
- just VERSION=vX.Y.Z release-check
```

## Phase 4: PR merge

1. CI が全て pass していることを確認する。
2. PR の全 commit が GitHub で `verified=true` / `reason=valid` であることを確認する。
3. 未検証 commit がある場合は merge せず、署名者と author / committer identity を修正してから CI を再実行する。
4. review comment がある場合は `github:gh-address-comments` skill で対応する。
5. `--admin` は使わない。
6. `gh pr merge --merge --delete-branch <PR番号またはURL>` で `main` に取り込む。
7. merge 後に `git switch main && git pull --ff-only origin main` を実行する。

## Phase 5: 公開

1. `main` が対象 version の commit を含むことを確認する。
2. `just VERSION=vX.Y.Z release` を実行する。
   - この target が signed annotated tag 作成、GitHub Verified 確認、Release workflow dispatch、crates.io publish dispatch を担当する。
3. Release workflow を監視する。

```bash
just release-status
gh run list --repo HiroyukiFuruno/katana-markdown-linter --workflow Release --limit 5
```

4. workflow 成功後、公開状態を検証する。

```bash
just VERSION=vX.Y.Z release-verify
```

## Phase 6: 事後整理

1. `branch-hygiene` skill を使う。
2. 削除対象:
   - merge 済みの `release/vX.Y.Z`
   - 使い終わった `feature/vX.Y.Z-*`
   - PR merge 時に削除対象だった remote branch
   - 不要な worktree
3. 残す対象:
   - `main`
   - 未コミット差分がある作業ブランチ
   - 次 version の `release/vX.Y.Z`
4. 最後に次を報告する。
   - GitHub Release URL
   - crates.io version
   - 削除した local branch
   - 削除した remote branch
   - 削除した worktree
   - 残した branch と理由

## 完了条件

- [ ] 対象 OpenSpec change の全 task が完了している
- [ ] `just VERSION=vX.Y.Z release-target-check` が成功している
- [ ] `just VERSION=vX.Y.Z release-check` が成功している
- [ ] release PR が `main` に merge されている
- [ ] `just VERSION=vX.Y.Z release` が成功している
- [ ] `just VERSION=vX.Y.Z release-verify` が成功している
- [ ] branch hygiene が完了している
- [ ] 次 patch / minor に回す課題が roadmap または OpenSpec change に残っている
