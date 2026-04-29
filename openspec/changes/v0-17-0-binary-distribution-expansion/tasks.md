# Tasks

## Definition of Ready

- [ ] 0.1 Homebrew tap の対象を、既存 `homebrew-katana` へ追加するか別 tap にするか決める
- [ ] 0.2 Homebrew formula 名を `kml` にするか `katana-markdown-linter` にするか決める
- [ ] 0.3 npm package 名と publish ownership を確認し、公開しない場合の deferred 表記を決める
- [ ] 0.4 PyPI package 名と publish ownership を確認し、公開しない場合の deferred 表記を決める
- [ ] 0.5 `v0.17.0` の supported target matrix を design の 4 target で確定する
- [ ] 0.6 release asset naming と checksum naming を README / docs に出せる形で確定する

## 1. Binary Packaging

- [ ] 1.1 `kml` release binary を target ごとに build する release script を追加する
- [ ] 1.2 Unix archive を `katana-markdown-linter-vX.Y.Z-<target>.tar.gz` で作る
- [ ] 1.3 Windows archive を `katana-markdown-linter-vX.Y.Z-<target>.zip` で作る
- [ ] 1.4 各 archive に `kml` / `kml.exe`、`LICENSE`、install note を含める
- [ ] 1.5 各 archive の `<archive>.sha256` を生成する
- [ ] 1.6 archive 展開後に `kml --version` と小さな `kml check` を実行する smoke test を追加する
- [ ] 1.7 `make binary-package` と `make binary-smoke` 相当の entrypoint を Makefile に追加する

## 2. Release Workflow

- [ ] 2.1 GitHub Actions release workflow に binary target matrix を追加する
- [ ] 2.2 matrix build が一部失敗したまま release asset を公開しないようにする
- [ ] 2.3 GitHub Release upload 対象に binary archive と checksum を追加する
- [ ] 2.4 `make release-check VERSION=v0.17.0` に binary package / smoke を組み込む
- [ ] 2.5 `make release-verify VERSION=v0.17.0` に binary asset と checksum の検証を追加する
- [ ] 2.6 release note 生成が binary asset の公開状態を説明できるようにする

## 3. Homebrew

- [ ] 3.1 Homebrew formula 生成 script を追加する
- [ ] 3.2 formula が release archive URL と SHA-256 checksum を参照することを検証する
- [ ] 3.3 formula の test block が `kml --version` を実行することを検証する
- [ ] 3.4 local で `brew audit` / `brew test` 相当を実行できる smoke path を作る
- [ ] 3.5 tap repository への差分作成手順を release runbook に追加する
- [ ] 3.6 tap への push / PR 作成を release 本体と混ぜず、検証後の独立手順にする

## 4. npm / pip Wrappers

- [ ] 4.1 npm wrapper の package metadata と install script 方針を追加する
- [ ] 4.2 npm wrapper が公式 binary archive を取得し checksum を検証することを smoke test する
- [ ] 4.3 pip wrapper の package metadata と install script 方針を追加する
- [ ] 4.4 pip wrapper が公式 binary archive を取得し checksum を検証することを smoke test する
- [ ] 4.5 wrapper publish enable flag と credential check を release workflow に追加する
- [ ] 4.6 publish しない wrapper を README / docs で公式導線として表示しない gate を追加する

## 5. Documentation and Release Metadata

- [ ] 5.1 `README.md` の install section を Cargo、GitHub Release binary、Homebrew、公開済み wrapper に分ける
- [ ] 5.2 `docs/distribution.md` の deferred table を `v0.17.0` の実装結果に合わせて更新する
- [ ] 5.3 `CHANGELOG.md` に `v0.17.0` の binary distribution 内容を追加する
- [ ] 5.4 `Cargo.toml` / `Cargo.lock` / `server.json` / MCPB manifest の version を `0.17.0` に更新する
- [ ] 5.5 `openspec/changes/active-roadmap.md` の `v0.17.0` 行を完了状態へ更新する

## 6. Verification

- [ ] 6.1 `make fmt-check`
- [ ] 6.2 `make lint`
- [ ] 6.3 `make ast-lint`
- [ ] 6.4 `cargo test --workspace --locked`
- [ ] 6.5 `make dogfood`
- [ ] 6.6 `git diff --check`
- [ ] 6.7 `make release-check VERSION=v0.17.0`
- [ ] 6.8 `make release-task-ledger-check VERSION=v0.17.0`

## Definition of Done

- [ ] 7.1 GitHub Release に supported target ごとの `kml` binary archive と checksum が添付される
- [ ] 7.2 local と CI の release gate が binary artifact を同じ script で検証する
- [ ] 7.3 Homebrew formula が release artifact と checksum を参照し、`kml --version` を検証する
- [ ] 7.4 npm / pip wrapper の公開または deferred 理由が tasks / docs / release note で一致している
- [ ] 7.5 Cargo install、GitHub Action、MCPB、Remote MCP の既存公式導線が壊れていない
- [ ] 7.6 対象 OpenSpec change が archive されている

## 品質評価スコア

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| binary artifact | 25 | 0 | 未実装。 |
| release workflow | 20 | 0 | 未実装。 |
| Homebrew | 20 | 0 | 未実装。 |
| npm / pip wrapper | 15 | 0 | 未実装。 |
| documentation | 10 | 0 | 未実装。 |
| verification | 10 | 0 | 未実装。 |
| 合計 | 100 | 0 | 実装前。 |
