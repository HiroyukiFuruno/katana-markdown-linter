# npm Package Polish Design

## Context

npm package `katana-markdown-linter@0.17.0` は公開済みだが、package page には README がなく、
keywords も空である。`v0.17.1` の npm publish は trusted publisher 設定不足で止まり、
その後 package 側の trusted publisher 設定は追加された。

一方で、README なしのまま `0.17.1` を公開すると、同じ version の内容を後から自然に直しにくい。
そのため `v0.17.2` を npm package polish patch とし、npm registry 上の見え方を整えてから
trusted publishing で公開する。

## Goals / Non-Goals

**Goals:**

- npm package page に README を表示し、導入コマンドと wrapper の役割を明確にする
- npm package metadata に search / support / repository に必要な情報を追加する
- package tarball に README と必要最小限の wrapper files だけが入ることを検証する
- trusted publishing で npm wrapper を公開し、公開後に `npm view` と `npx` を確認する
- `v0.17.1` の npm 未完了状態を release ledger から `v0.17.2` に引き継ぐ

**Non-Goals:**

- npm wrapper に lint rule / formatter / LSP logic を追加すること
- npm package に不要な runtime dependency を追加すること
- crates.io / PyPI / Homebrew の再設計
- `v0.18.0` schema publication や editor extension 計画を混ぜること

## Decisions

### D-1: npm README は root README の完全コピーにしない

`wrappers/npm/README.md` は npm package page 専用の短い文書にする。
root `README.md` は product 全体の説明を持つため、npm package にそのまま入れると
Cargo / Homebrew / PyPI / GitHub Action まで混ざり、npm user が必要な情報を探しにくい。

README には次を含める。

- `npm install -g katana-markdown-linter`
- `npx --yes katana-markdown-linter@<version> --version`
- `kml check README.md` の最小例
- thin wrapper が GitHub Release binary を取得し、checksum を検証すること
- supported platforms と unsupported platform の扱い
- issue / repository link

### D-2: dependencies が 0 であることは維持する

npm wrapper は Node.js 標準 module と `curl` / `tar` だけを使う。
package page の dependencies が 0 であることは品質上の問題ではなく、
thin wrapper としての attack surface を小さくする意図に合う。

### D-3: package metadata は npm page の探索性を補う範囲に限定する

`keywords`、`homepage`、`bugs` を追加し、repository URL は npm が正規化できる形式にする。
metadata 追加は user-facing package page の改善に限定し、package name や bin name は変えない。

### D-4: tarball 内容は release gate で固定する

`npm pack --dry-run --json` の結果に、少なくとも次が含まれることを検証する。

- `README.md`
- `package.json`
- `bin/kml.js`
- `lib/installer.js`

逆に、Cargo build artifact や repository root の重いファイルは含めない。

### D-5: v0.17.2 は npm publish closeout の patch とする

`v0.17.1` で GitHub Release / crates.io / PyPI / Homebrew が先に完了したため、
`v0.17.2` は npm package page と npm publish の完了に絞る。
`v0.18.0` 以降の schema / editor work は、npm channel の visible gap を閉じた後に再開する。

## Risks / Trade-offs

- root README と npm README の重複が増える
  - npm README は短く保ち、詳細は root README / docs へ誘導する
- npm trusted publishing は package 側設定に依存する
  - release 前に trusted publisher entry を確認し、workflow filename を `release.yml` に固定する
- README を package `files` に入れ忘れる
  - `npm pack --dry-run --json` を release gate と AST lint の確認対象にする
- `npx` が cache 済み binary を使って誤判定する
  - smoke test では fresh install directory を使う

## Migration Plan

1. v0.17.1 の npm 未完了理由を v0.17.2 tasks に引き継ぐ
2. `wrappers/npm/README.md` と npm package metadata を追加する
3. npm tarball content check を local gate または AST lint に追加する
4. wrapper smoke が fresh install directory で release version を返すことを確認する
5. `v0.17.2` metadata / changelog / docs を更新する
6. trusted publishing で npm wrapper を公開する
7. `make release-verify VERSION=v0.17.2` または npm-focused verification で registry state を確認する

## Closed Questions

- npm README は今回は root README と完全同期しない。npm package page 専用の短い文書として維持する。
- `v0.17.2` は `make release VERSION=v0.17.2` の通常手順で GitHub Release / crates.io / npm / PyPI を同一 version として公開し、npm wrapper だけの特殊 retry にはしない。
