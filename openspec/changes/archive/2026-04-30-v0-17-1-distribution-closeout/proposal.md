# Distribution Closeout

## Target Version

`v0.17.1`

## Why

`v0.17.0` で GitHub Release、crates.io、npm、PyPI への公開は完了した。
一方で README / docs / release verification / Homebrew tap / npm token 後始末がまだ
公開済み状態に追いついていない。

この change では、公開済み wrapper を公式 install channel として扱える状態にし、
次の開発へ進む前に配布導線の運用差分を閉じる。

## What Changes

- npm / PyPI wrapper を公式 install channel として README と docs に反映する
- Homebrew formula を `homebrew-katana` tap へ反映するための差分と検証手順を固定する
- npm publish を一時的な `NPM_TOKEN` 依存から trusted publishing 前提へ戻す
- PyPI trusted publisher 設定と GitHub Actions の `pypi` environment 前提を release docs に固定する
- `make release-verify` が npm、PyPI、wrapper 起動、Homebrew formula まで確認するようにする
- `docs/distribution.md` と `docs/release-runbook.md` の deferred 表記を公開済み状態に合わせる

## Capabilities

### New Capabilities

なし。

### Modified Capabilities

- `binary-distribution`: npm / PyPI wrapper と Homebrew formula の公式化条件を更新する
- `release-cicd`: wrapper publish job と trusted publishing の責務を更新する
- `release-readiness`: post-release verification の対象を external registry と tap まで広げる

## Impact

- `.github/workflows/release.yml`
- `Makefile`
- `scripts/release/verify-release-published.sh`
- `scripts/release/wrapper-publish-gate.sh`
- `scripts/release/homebrew_formula.py`
- `wrappers/npm/**`
- `wrappers/python/**`
- `README.md`
- `docs/distribution.md`
- `docs/release-runbook.md`
- `docs/quality-gates.md`
- `CHANGELOG.md`
- sibling repository: `/Users/hiroyuki_furuno/works/private/homebrew-katana`
