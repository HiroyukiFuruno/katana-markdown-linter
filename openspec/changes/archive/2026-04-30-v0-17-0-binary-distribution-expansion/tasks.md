# Tasks

## Definition of Ready

- [x] 0.1 Homebrew tap の対象は既存 `homebrew-katana` にする
- [x] 0.2 Homebrew formula 名は `kml` を第一候補にし、衝突する場合は `katana-markdown-linter` または `katana-ml` にする
- [x] 0.3 npm package 名は `katana-markdown-linter` が未登録であることを確認し、npm account / trusted publishing 未準備のため publish は deferred とする
- [x] 0.4 PyPI package 名は `katana-markdown-linter` が未登録であることを確認し、PyPI account / trusted publishing 未準備のため publish は deferred とする
- [x] 0.5 `v0.17.0` の supported target matrix は design の 4 target を MVP scope として確定する
- [x] 0.6 release asset naming は `kml-vX.Y.Z-<target>.tar.gz`、Windows は `kml-vX.Y.Z-<target>.zip`、checksum naming は `<archive>.sha256` にする
- [x] 0.7 npm trusted publishing がない場合は wrapper publish を deferred にする前提を release gate に記録する
- [x] 0.8 PyPI trusted publishing がない場合は wrapper publish を deferred にする前提を release gate に記録する
- [x] 0.9 GitHub の秘匿変数（Secrets）に `NPM_TOKEN` と `PYPI_API_TOKEN` を要求せず、trusted publishing の OIDC publish job だけで publish する

### Task 0 ユーザー実施内容

`v0.17.0` では npm / PyPI wrapper publish を deferred とするため、この項目は release blocker ではない。
今回の release で npm / PyPI wrapper まで公式公開する場合だけ、ユーザーが account 作成と trusted publishing 設定を実施する。

#### npm

- npm account を作成し、公開に使う account で login できる状態にする。
- GitHub 連携を有効化する。
- trusted publishing は次の値で設定する。
  - package: `katana-markdown-linter`
  - owner: `HiroyukiFuruno`
  - repository: `katana-markdown-linter`
  - workflow filename: `release.yml`
  - environment: 未指定

#### PyPI

- PyPI account を作成し、email verification と二要素認証（2FA）を完了する。
- GitHub repository 側に `pypi` environment を作成する。
- pending publisher は次の値で設定する。
  - PyPI project name: `katana-markdown-linter`
  - owner: `HiroyukiFuruno`
  - repository: `katana-markdown-linter`
  - workflow name: `release.yml`
  - environment name: `pypi`
- PyPI project name は `wrappers/python/pyproject.toml` の package 名と一致させる。`katana-ml` に変える場合は wrapper metadata を先に変更する。

#### GitHub Actions 実行

~~~bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-linter

gh workflow run release.yml \
  --repo HiroyukiFuruno/katana-markdown-linter \
  --field version=v0.17.0 \
  --field publish_npm_wrapper=true \
  --field publish_pypi_wrapper=true
~~~

## 1. Binary Packaging

- [x] 1.1 `kml` release binary を target ごとに build する release script を追加する
- [x] 1.2 Unix archive を `kml-vX.Y.Z-<target>.tar.gz` で作る
- [x] 1.3 Windows archive を `kml-vX.Y.Z-<target>.zip` で作る
- [x] 1.4 各 archive に `kml` / `kml.exe`、`LICENSE`、install note を含める
- [x] 1.5 各 archive の `<archive>.sha256` を生成する
- [x] 1.6 archive 展開後に `kml --version` と小さな `kml check` を実行する smoke test を追加する
- [x] 1.7 `make binary-package` と `make binary-smoke` 相当の entrypoint を Makefile に追加する

## 2. Release Workflow

- [x] 2.1 GitHub Actions release workflow に binary target matrix を追加する
- [x] 2.2 matrix build が一部失敗したまま release asset を公開しないようにする
- [x] 2.3 GitHub Release upload 対象に binary archive と checksum を追加する
- [x] 2.4 `make release-check VERSION=v0.17.0` に binary package / smoke を組み込む
- [x] 2.5 `make release-verify VERSION=v0.17.0` に binary asset と checksum の検証を追加する
- [x] 2.6 release note 生成が binary asset の公開状態を説明できるようにする

## 3. Homebrew

- [x] 3.1 Homebrew formula 生成 script を追加する
- [x] 3.2 formula が release archive URL と SHA-256 checksum を参照することを検証する
- [x] 3.3 formula の test block が `kml --version` を実行することを検証する
- [x] 3.4 local で `brew audit` / `brew test` 相当を実行できる smoke path を作る
- [x] 3.5 tap repository への差分作成手順を release runbook に追加する
- [x] 3.6 tap への push / PR 作成を release 本体と混ぜず、検証後の独立手順にする

## 4. npm / pip Wrappers

- [x] 4.1 npm wrapper の package metadata と install script 方針を追加する
- [x] 4.2 npm wrapper が公式 binary archive を取得し checksum を検証することを smoke test する
- [x] 4.3 pip wrapper の package metadata と install script 方針を追加する
- [x] 4.4 pip wrapper が公式 binary archive を取得し checksum を検証することを smoke test する
- [x] 4.5 wrapper publish enable flag と trusted publishing job を release workflow に追加する
- [x] 4.6 publish しない wrapper を README / docs で公式導線として表示しない gate を追加する

## 5. Documentation and Release Metadata

- [x] 5.1 `README.md` の install section を Cargo、GitHub Release binary、Homebrew、公開済み wrapper に分ける
- [x] 5.2 `docs/distribution.md` の deferred table を `v0.17.0` の実装結果に合わせて更新する
- [x] 5.3 `CHANGELOG.md` に `v0.17.0` の binary distribution 内容を追加する
- [x] 5.4 `Cargo.toml` / `Cargo.lock` / `server.json` / MCPB manifest の version を `0.17.0` に更新する
- [x] 5.5 `openspec/changes/active-roadmap.md` の `v0.17.0` 行を完了状態へ更新する

## 6. Verification

- [x] 6.1 `make fmt-check`
- [x] 6.2 `make lint`
- [x] 6.3 `make ast-lint`
- [x] 6.4 `cargo test --workspace --locked`
- [x] 6.5 `make dogfood`
- [x] 6.6 `git diff --check`
- [x] 6.7 `make release-check VERSION=v0.17.0`
- [x] 6.8 `make release-task-ledger-check VERSION=v0.17.0`

## Definition of Done

- [x] 7.1 GitHub Release に supported target ごとの `kml` binary archive と checksum が添付される
- [x] 7.2 local と CI の release gate が binary artifact を同じ script で検証する
- [x] 7.3 Homebrew formula が release artifact と checksum を参照し、`kml --version` を検証する
- [x] 7.4 npm / pip wrapper の公開または deferred 理由が tasks / docs / release note で一致している
- [x] 7.5 Cargo install、GitHub Action、MCPB、Remote MCP の既存公式導線が壊れていない
- [x] 7.6 対象 OpenSpec change が archive されている

## 品質評価スコア

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| binary artifact | 25 | 25 | `make binary-smoke VERSION=v0.17.0` と release workflow matrix を実装済み。 |
| release workflow | 20 | 20 | binary asset upload、checksum、`make release-verify` 検証を実装済み。 |
| Homebrew | 20 | 20 | formula 生成と `homebrew-formula-check`、tap 更新手順を実装済み。 |
| npm / pip wrapper | 15 | 15 | wrapper source、local smoke、trusted publishing gate、deferred 表記を実装済み。 |
| documentation | 10 | 10 | README、distribution、release runbook、CHANGELOG を更新済み。 |
| verification | 10 | 10 | `make release-check VERSION=v0.17.0` が成功。 |
| 合計 | 100 | 100 | release-ready。 |
