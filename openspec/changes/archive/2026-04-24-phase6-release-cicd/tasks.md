## Definition of Ready

- [x] phase1 から phase5 の `tasks.md` が全て完了していること
- [x] GitHub tag / release が現状存在しないことを確認していること
- [x] Cargo package version を release version の source of truth にすることが確定していること
- [x] crates.io publish には `CARGO_REGISTRY_TOKEN` secret が必要であることが明示されていること
- [x] KatanA 本体の release workflow を参考にしつつ、この crate に不要な desktop artifact 配布は scope 外にしていること

## 1. Release Metadata

- [x] 1.1 `CHANGELOG.md` を追加し、初期 version の release notes を記載する
- [x] 1.2 README / runbook に GitHub Release と crates.io publish の関係を記載する
- [x] 1.3 Cargo version と release version の一致を検証する helper を追加する

## 2. Release Workflow

- [x] 2.1 `workflow_dispatch` で `version` と `publish_crate` を受け取る release workflow を追加する
- [x] 2.2 `push.tags: v*` でも release workflow が動くようにする
- [x] 2.3 release workflow 内で fmt / test / clippy / upstream drift / package dry-run / install smoke test を実行する
- [x] 2.4 GitHub Release と `vX.Y.Z` tag を作成または更新する
- [x] 2.5 `.crate` package と checksum を GitHub Release artifact として添付する
- [x] 2.6 `publish_crate` が true の場合に crates.io publish を実行する

## 3. Release Safety

- [x] 3.1 Cargo version と release input / tag が不一致なら失敗する
- [x] 3.2 crates.io token がない状態で publish を要求した場合は失敗理由を明示する
- [x] 3.3 release runbook に失敗時の復旧手順を追記する

## Definition of Done

- [x] GitHub Actions 上で release flow が version validation から GitHub Release 作成まで表現されていること
- [x] crates.io publish は dry-run が必須 gate になっていること
- [x] `cargo install` 可能性が release gate で確認されること
- [x] release 手順が README / runbook / CHANGELOG から追跡できること
