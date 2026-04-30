# Distribution Closeout Design

## Context

`v0.17.0` の wrapper 公開後、npm と PyPI は実際に利用できる状態になった。
しかし `docs/distribution.md` には deferred 表記が残り、`release-verify` は
GitHub Release / crates.io / binary assets 中心の確認に留まっている。

npm の初回公開では `NPM_TOKEN` を使ったため、次の release までに
trusted publishing へ戻し、token を不要にする必要がある。
PyPI は trusted publisher が設定済みで、GitHub Actions 側は `pypi` environment を使う。

## Goals / Non-Goals

**Goals:**

- npm / PyPI wrapper を公式 install channel として docs に反映する
- npm publish job から `NPM_TOKEN` 前提を取り除く
- `release-verify` で npm / PyPI / wrapper 起動 / Homebrew formula を確認する
- `homebrew-katana` tap 更新を release 本体と分離した review 可能な差分にする
- 公式導線と検証 gate の説明を README / docs / changelog で一致させる

**Non-Goals:**

- wrapper に Markdown lint logic を実装すること
- package manager ごとに CLI option や exit code を変えること
- Homebrew tap を未検証のまま自動 push すること
- `kml lsp` や editor extension の機能追加
- 新しい distribution channel の追加

## Decisions

### D-1: wrapper は公式 binary の launcher のまま維持する

npm / PyPI package は、引き続き GitHub Release の binary archive を取得して
`kml` を起動する薄いラッパー（thin wrapper）にする。
wrapper 側に rule、formatter、LSP logic は持たせない。

### D-2: npm publish は trusted publishing を標準に戻す

`NPM_TOKEN` は初回公開用の暫定手段だった。
`v0.17.1` では npm package 側の trusted publishing 設定を前提にし、
GitHub Actions の publish job は OIDC を使う。
token が必要な経路は release docs から通常手順として削除する。

### D-3: post-release verification は外部 registry まで見る

`make release-verify VERSION=vX.Y.Z` は次を確認する。

- GitHub Release の tag / title / target / draft 状態
- supported target の binary archive と checksum
- crates.io version
- npm package version と `npx --yes katana-markdown-linter@X.Y.Z --version`
- PyPI JSON version と `uvx --from katana-markdown-linter==X.Y.Z kml --version`
- generated Homebrew formula の version、URL、checksum、test block

### D-4: Homebrew tap 更新は別 repository の review flow に残す

この repository は formula 生成と検証を担当する。
`homebrew-katana` への反映は sibling repository で差分を作り、
`brew audit` / `brew test` 相当の結果を確認してから commit / push する。

### D-5: docs は公開済み channel だけを公式として書く

README と docs は、実際に verified できる channel だけを公式導線として扱う。
deferred / provisional / token-required の古い表現は残さない。

## Risks / Trade-offs

- npm trusted publishing の設定値が workflow filename とずれる
  - `docs/release-runbook.md` と release workflow の workflow filename を一致させる
- `uvx` や `npx` の network 状態で verification が失敗する
  - `release-verify` は post-release gate とし、失敗時は registry 状態を明示して止める
- Homebrew tap 更新が別 repository の状態に影響される
  - formula 生成結果と tap 差分を分け、tap 側で通常の branch / PR flow を使う
- token cleanup を急ぎすぎると次回 npm publish が止まる
  - npm trusted publishing 設定の存在を確認してから `NPM_TOKEN` 依存を削除する

## Migration Plan

1. npm trusted publishing 設定値を docs と workflow で固定する
2. `release.yml` の npm publish job から `NODE_AUTH_TOKEN` / `NPM_TOKEN` 依存を取り除く
3. `release-verify` に npm / PyPI / wrapper launch / Homebrew formula checks を追加する
4. Homebrew formula を `homebrew-katana` tap に反映する差分を作る
5. README / distribution docs / release runbook / quality gates / changelog を更新する
6. `v0.17.1` release check と post-release verification を実行する

## Open Questions

- npm trusted publishing の package 側設定が完了しているかは、implementation 開始時に確認する
- Homebrew tap 更新は direct push か PR か、tap repository の branch protection を見て決める
