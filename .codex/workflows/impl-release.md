---
description: 指定バージョンのOpenSpec実装、品質確認、リリース準備、PR作成、GitHub Release、crates.io publish、自己レビュー、事後整理までを自律的に遂行する Implementation & Release Autopilot ワークフロー。
---

# /impl-release vX.Y.Z

指定バージョンの OpenSpec change に基づく実装・修正から、リリース準備、PR 作成、GitHub Release、crates.io publish、release verification、branch hygiene までを一気通貫で進める。

## 前提

- 対象バージョンはユーザー入力から受け取る。例: `/impl-release v0.12.2`
- 作業対象 repository は `katana-markdown-linter`
- default branch は `main`
- release は `make release VERSION=vX.Y.Z` を正とする
- publish 後 verification は `make release-verify VERSION=vX.Y.Z` を正とする
- `make release` が失敗した場合、手動 tag 作成や `cargo publish` 直叩きで迂回しない

## 停止ルール

この workflow は release 完了まで自律的に進める。次の条件に当たる場合だけ作業を中断し、ユーザーの判断を仰ぐ。

- OpenSpec tasks に記載がない不足や想定外が進行中に露見した
- その不足を Codex の判断だけで補うと、公開物、API、互換性、release 成果物、または既存作業を壊すリスクが高い

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
4. 対象 version の active OpenSpec change を特定する。
   - 例: `v0.12.2` -> `openspec/changes/v0-12-2-*`
   - 見つからない場合は `openspec/changes/active-roadmap.md` を確認する。
5. 対象 change の `proposal.md`、`design.md`、`tasks.md`、`specs/**/spec.md` を読む。

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
make fmt-check
make lint
make ast-lint
cargo test --workspace --locked
make dogfood
git diff --check
```

release 前 gate:

```bash
make release-check VERSION=vX.Y.Z
```

対象 change の `tasks.md` に追加 verification がある場合は、それも実行する。

## Phase 3: リリース準備 PR

1. `Cargo.toml` / `Cargo.lock` / README / docs / CHANGELOG / OpenSpec archive を対象 version に合わせる。
2. public docs は英語で書く。`README.md` または `docs/**` を変更した場合は `make ast-lint` を実行する。
3. 対象 OpenSpec change の全 task が完了したら、`openspec-archive-change` skill に従って archive する。
4. commit 前に `git status --short --branch` と `git diff --cached --stat` を確認する。
5. PR を作成し、CI を監視する。

推奨 PR body:

```markdown
## Summary
- Prepare vX.Y.Z release
- Complete <change-id>
- Archive completed OpenSpec change

## Verification
- make release-check VERSION=vX.Y.Z
```

## Phase 4: PR merge

1. CI が全て pass していることを確認する。
2. review comment がある場合は `github:gh-address-comments` skill で対応する。
3. `--admin` は使わない。
4. 通常 merge で `main` に取り込む。
5. merge 後に `git switch main && git pull --ff-only origin main` を実行する。

## Phase 5: 公開

1. `main` が対象 version の commit を含むことを確認する。
2. `make release VERSION=vX.Y.Z` を実行する。
   - この target が signed annotated tag 作成、GitHub Verified 確認、Release workflow dispatch、crates.io publish dispatch を担当する。
3. Release workflow を監視する。

```bash
make release-status
gh run list --repo HiroyukiFuruno/katana-markdown-linter --workflow Release --limit 5
```

4. workflow 成功後、公開状態を検証する。

```bash
make release-verify VERSION=vX.Y.Z
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
- [ ] `make release-check VERSION=vX.Y.Z` が成功している
- [ ] release PR が `main` に merge されている
- [ ] `make release VERSION=vX.Y.Z` が成功している
- [ ] `make release-verify VERSION=vX.Y.Z` が成功している
- [ ] branch hygiene が完了している
- [ ] 次 patch / minor に回す課題が roadmap または OpenSpec change に残っている
