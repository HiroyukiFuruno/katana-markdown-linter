## Why

`katana-markdown-linter` は publish-ready な crate になっているが、GitHub 上には tag / release が存在せず、version を外部から確認できない。
phase6 は、push 後の GitHub 品質ゲートを前提に、version validation、GitHub Release、crates.io publish を CI/CD 上の release flow として確立する。

## What Changes

- Cargo package version を release version の source of truth とする
- GitHub Actions で release workflow を追加する
- release 前に existing quality gates、upstream drift check、package dry-run、install smoke test を実行する
- GitHub Release と `vX.Y.Z` tag を作成する
- `CARGO_REGISTRY_TOKEN` が設定された環境で crates.io publish を実行できるようにする
- release notes と release runbook を整備する

## Impact

- GitHub 上で version / release が確認できるようになる
- manual release と tag push release の両方を CI/CD 管理できる
- publish 前に品質ゲートが通らない release を止められる
