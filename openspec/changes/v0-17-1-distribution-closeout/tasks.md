# Tasks

## Definition of Ready

- [ ] 0.1 npm package `katana-markdown-linter` の trusted publishing 設定を確認する
- [ ] 0.2 PyPI project `katana-markdown-linter` の trusted publisher と `pypi` environment を確認する
- [ ] 0.3 `homebrew-katana` tap の local checkout と branch protection を確認する
- [ ] 0.4 `v0.17.0` の npm / PyPI / GitHub Release / crates.io 公開状態を再確認する
- [ ] 0.5 `NPM_TOKEN` を次回 release の通常手順から外せることを確認する

## 1. npm / PyPI Wrapper Officialization

- [ ] 1.1 npm wrapper の install command と `npx` 実行例を README / docs に公式導線として反映する
- [ ] 1.2 PyPI wrapper の install command と `uvx` 実行例を README / docs に公式導線として反映する
- [ ] 1.3 wrapper が binary launcher であり、独自 lint logic を持たないことを docs に明記する
- [ ] 1.4 `docs/distribution.md` の npm / pip deferred 表記を公開済み状態へ更新する

## 2. Trusted Publishing Cleanup

- [ ] 2.1 `.github/workflows/release.yml` の npm publish job から `NPM_TOKEN` / `NODE_AUTH_TOKEN` 依存を削除する
- [ ] 2.2 npm publish job が trusted publishing 前提で失敗する場合の error message を明確にする
- [ ] 2.3 PyPI publish job が `pypi` environment と OIDC publish を維持していることを検証する
- [ ] 2.4 `scripts/release/wrapper-publish-gate.sh` の文言と実際の workflow 条件を一致させる
- [ ] 2.5 `docs/release-runbook.md` から初回公開用 token 手順を通常手順として扱う記述を外す

## 3. Release Verification

- [ ] 3.1 `scripts/release/verify-release-published.sh` に npm registry version check を追加する
- [ ] 3.2 `scripts/release/verify-release-published.sh` に PyPI JSON version check を追加する
- [ ] 3.3 `scripts/release/verify-release-published.sh` に `npx` wrapper smoke を追加する
- [ ] 3.4 `scripts/release/verify-release-published.sh` に `uvx` wrapper smoke を追加する
- [ ] 3.5 `scripts/release/verify-release-published.sh` に Homebrew formula URL / checksum / test block check を追加する
- [ ] 3.6 `make release-verify VERSION=v0.17.1` が追加検証を呼ぶことを確認する

## 4. Homebrew Tap Update

- [ ] 4.1 `make homebrew-formula VERSION=v0.17.1` で生成される formula を確認する
- [ ] 4.2 `homebrew-katana` tap に `Formula/kml.rb` 差分を作る
- [ ] 4.3 tap 側で `brew audit` / `brew test` 相当の確認を行う
- [ ] 4.4 tap 更新の commit / push / PR 方針を branch protection に合わせて実行する
- [ ] 4.5 tap 更新結果を release runbook に追記する

## 5. Documentation and Release Metadata

- [ ] 5.1 README の install section を Cargo / GitHub Release / npm / PyPI / Homebrew の現状に合わせる
- [ ] 5.2 `docs/release-runbook.md` の wrapper publish と post-release verification を更新する
- [ ] 5.3 `docs/quality-gates.md` に registry / wrapper / Homebrew verification を追加する
- [ ] 5.4 `CHANGELOG.md` に `v0.17.1` の配布後始末を追加する
- [ ] 5.5 version metadata を `0.17.1` に更新する

## 6. Verification

- [ ] 6.1 `make fmt-check`
- [ ] 6.2 `make lint`
- [ ] 6.3 `make ast-lint`
- [ ] 6.4 `cargo test --workspace --locked`
- [ ] 6.5 `make dogfood`
- [ ] 6.6 `git diff --check`
- [ ] 6.7 `make release-check VERSION=v0.17.1`
- [ ] 6.8 `make release-task-ledger-check VERSION=v0.17.1`

## Definition of Done

- [ ] 7.1 npm / PyPI wrapper が README と docs で公式 install channel として扱われている
- [ ] 7.2 npm publish job が通常 release path で `NPM_TOKEN` を要求しない
- [ ] 7.3 `make release-verify` が npm / PyPI / wrapper launch / Homebrew formula を確認する
- [ ] 7.4 Homebrew tap 更新が review 可能な差分として完了している
- [ ] 7.5 `v0.17.1` release 後の public registry state が verification で確認できる
