## Definition of Ready

- [x] `v0.3.0` の GitHub Release と crates.io publish が成功済みであること
- [x] working tree の tracked 差分がこの change に採用するものか、別作業として除外するものか明確であること
- [x] active release 作業が存在しないこと
- [x] `harden-check-fix-parity` が archive 済み、または全 task 完了済みであること
- [x] GitHub 上で `v0.3.0` tag が `Verified` であることを確認済みであること

## 1. Current Release State Visibility

- [x] 1.1 `make release-status` で直近 Release workflow の success/failure が確認できることを検証する
- [x] 1.2 `v0.3.0` tag target、GitHub Release target、crates.io version の一致を記録する
- [x] 1.3 `v0.2.0` release notes 補正済みであることを確認する
- [x] 1.4 GitHub API で release tag の verification state を確認する

Evidence:

- `make release-status` displays recent Release workflow success/failure rows.
- `make release-verify VERSION=v0.3.0` reports `tag_target=12b7a6b855b8456c1aa3a2e02b8b40128cd1ba8c`, `github_release_target=12b7a6b855b8456c1aa3a2e02b8b40128cd1ba8c`, and `crates_io_version=0.3.0`.
- `scripts/release/verify-tag-verified.sh v0.3.0 HiroyukiFuruno/katana-markdown-linter` reports GitHub `Verified`.
- `v0.2.0` GitHub Release notes were repopulated from `CHANGELOG.md`.

## 2. Local/CI Gate Parity

- [x] 2.1 `make lint` が release workflow の Clippy command と一致していることを test または AST lint で固定する
- [x] 2.2 release workflow に gate を追加した場合に Makefile 側が追従していることを検出する
- [x] 2.3 examples / tests / optional feature targets が local release-check から漏れないことを確認する
- [x] 2.4 GitHub `Verified` tag check が local release-tag と release workflow の両方で実行されることを固定する

## 3. Retry Safety

- [x] 3.1 tag rewrite が許可される条件を docs または Makefile help に明記する
- [x] 3.2 release 済み tag を誤って上書きしない fail-fast check を追加する
- [x] 3.3 crates.io publish 済み version で `make release` した場合に安全に停止する
- [x] 3.4 failed workflow のみ存在する version と published version の判定を分ける

## 4. Release Commands

- [x] 4.1 GitHub Release のみ作成する command と crates.io publish まで行う command の責務を README / Makefile に明記する
- [x] 4.2 release 後 verification command を Makefile に追加する
- [x] 4.3 release notes 生成が changelog section 欠落時に fail-fast することを確認する

## Verification

- [x] `make release-check VERSION=v0.3.0` が成功する
- [x] `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` が成功する
- [x] `make release-status` が release 状態を表示する
- [x] `scripts/release/verify-tag-verified.sh v0.3.0 HiroyukiFuruno/katana-markdown-linter` が成功する
- [x] `git diff --check` が成功する

## Definition of Done

- [x] local release preflight と CI release gate の差分が可視化されていること
- [x] release retry の安全条件が機械的または手順として明確であること
- [x] GitHub Release / crates.io / tag target の整合確認が標準手順に含まれていること
- [x] GitHub `Verified` tag を release の必須条件として固定していること
