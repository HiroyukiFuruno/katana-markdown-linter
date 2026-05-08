# Design

## 方針

`v0.20.0` は editor marketplace 公開専用の change とする。editor 機能の不足修正や dogfood のバグ修正は、この change へ混ぜない。`v0-19-0-editor-capability-completion` が完了し、final editor dogfood の release-blocking finding が 0 件であることを DoR にする。

## 公開前提

- `v0-19-0-editor-capability-completion` が完了している。
- final editor dogfood evidence が存在し、診断・整形・安全な修正・config changes を含む。
- VS Code publisher、extension name、token secret が確認済み。
- Zed extension registry PR が merged である。
- target version が GitHub Release、crates.io、npm、PyPI、Homebrew、VS Code Marketplace、Zed registry のいずれにも既存ではない。

## Release workflow

- editor marketplace publish は明示 input / env が有効な場合だけ実行する。
- VS Code publish は `VSCE_PAT` secret がない場合に fail fast する。
- Zed publish は upstream registry PR URL がない、または未mergeの場合に fail fast する。
- publish job は core release の状態と分離して記録し、partial publish の状態を release verification で説明する。

## CI/CD variable contract

`v0.20.0` で使う公開用の値は以下に固定する。実装中に追加の secret / env / workflow input が必要に見えた場合は、自己判断で増やさず設計差分として報告する。

| 種別 | 名前 | 用途 | 必須条件 |
| --- | --- | --- | --- |
| workflow input | `publish_vscode_extension` | VS Code Marketplace publish を明示的に有効化する | default は `false`。`true` の場合だけ publish job を実行する |
| workflow input | `publish_zed_extension` | Zed registry verification を明示的に有効化する | default は `false`。`true` の場合だけ Zed publication verification を必須化する |
| workflow input | `zed_extension_publication_pr_url` | Zed registry 公開 PR を指定する | `publish_zed_extension=true` の場合だけ必須 |
| local env | `PUBLISH_VSCODE_EXTENSION` | local script / `just release-verify` 用の VS Code publish flag | `true` / `false` のみ。未指定は `false` |
| local env | `PUBLISH_ZED_EXTENSION` | local script / `just release-verify` 用の Zed publish flag | `true` / `false` のみ。未指定は `false` |
| local env | `ZED_EXTENSION_PUBLICATION_PR_URL` | Zed registry 公開 PR の URL | `PUBLISH_ZED_EXTENSION=true` の場合だけ必須 |
| repository secret | `VSCE_PAT` | `vsce publish` 用の Visual Studio Marketplace token | `PUBLISH_VSCODE_EXTENSION=true` の場合だけ必須 |
| GitHub-provided token | `GITHUB_TOKEN` / `GH_TOKEN` | GitHub Release と Zed PR 状態の読み取り・検証 | workflow 内で GitHub API / `gh pr view` に使う。Marketplace credential として使わない |

### 採用しない値

- `ACTIONS_ID_TOKEN_REQUEST_TOKEN` / `ACTIONS_ID_TOKEN_REQUEST_URL` は VS Code Marketplace publish の認証条件にしない。Microsoft の VS Code Marketplace は `vsce` publish に Personal Access Token を要求するため、VS Code では `VSCE_PAT` を使う。
- `NPM_TOKEN` / `NODE_AUTH_TOKEN` / `PYPI_API_TOKEN` は editor extension publish に使わない。npm / PyPI wrapper の trusted publishing と混ぜない。
- `ZED_EXTENSION_TOKEN` のような独自 secret は作らない。Zed は upstream registry PR merge を公開完了条件にする。

## Workflow input mapping

`workflow_dispatch` では以下の対応で env へ渡す。

- `inputs.publish_vscode_extension` -> `PUBLISH_VSCODE_EXTENSION`
- `inputs.publish_zed_extension` -> `PUBLISH_ZED_EXTENSION`
- `inputs.zed_extension_publication_pr_url` -> `ZED_EXTENSION_PUBLICATION_PR_URL`

release PR merge 経由では editor publication を自動実行しない。core release は editor publication を `deferred` として記録し、Marketplace 公開は明示 input 付きの workflow dispatch または同等の明示 local command で扱う。

## VS Code publication

VS Code Marketplace は `vsce` の publish path を使う。`VSCE_PAT` は repository secret とし、workflow input や local env に token 値を入れない。

実行時の前提は以下にする。

- `publish_vscode_extension=true` または `PUBLISH_VSCODE_EXTENSION=true` が明示されている。
- `VSCE_PAT` が GitHub Actions secret として存在する。
- `editors/vscode/package.json` の `publisher` / `name` / `version` が target version と一致する。
- publish 前に target version が VS Code Marketplace に存在しないことを `release-target-check` で確認済み。

## Zed publication

Zed extension registry への公開は、`zed-industries/extensions` の PR merge によって成立する。この repository の release workflow は Zed registry へ直接 publish しない。

実行時の前提は以下にする。

- `publish_zed_extension=true` または `PUBLISH_ZED_EXTENSION=true` が明示されている。
- `ZED_EXTENSION_PUBLICATION_PR_URL` が `https://github.com/zed-industries/extensions/pull/<number>` を指す。
- `gh pr view` または GitHub API で PR が merged であることを確認できる。
- PR 内の `extensions.toml` version と `editors/zed/extension.toml` version が target version と一致する。

## Verification

- `release-target-check` は target version の既存公開状態を editor marketplaces まで確認する。
- `release-verify` は publication flag に応じて `published` / `deferred` / `failed` を説明する。
- VS Code Marketplace と Zed registry の両方で target version が確認できるまで、`v0.20.0` は完了扱いにしない。

## 非対象

- LSP config 解決。
- editor diagnostics / quick fix の挙動修正。
- final editor dogfood で見つかった release-blocking issue の修正。
- `v0.18.7` 事故版の再公開。

## Rollback / retry

- 既に外部 registry に公開された version は同じ意味で再利用しない。
- 片方の marketplace だけ公開された場合、状態を evidence に残し、同一 version で安全に retry できるかを registry ごとに確認する。
- 内容変更が必要な場合は、次 version へ進める。
