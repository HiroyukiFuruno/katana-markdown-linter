# Binary Distribution Expansion Design

## Context

`v0.16.1` 時点の公式導線は Cargo crate、GitHub Action、MCPB、Remote MCP、editor / LSP である。
一方、`docs/distribution.md` では Homebrew、単体バイナリ配布物（standalone binary artifact）、npm ラッパー（npm wrapper）、pip / uv ラッパー（pip/uv wrapper）が deferred になっている。

この change は、Rust toolchain を直接使わない利用者向けに `kml` を配るための release contract を作る。
配布対象は CLI binary の `kml` であり、`kml-mcp` は既存 MCPB 導線に残す。

## Goals / Non-Goals

**Goals:**

- GitHub Release に OS / architecture 別の `kml` binary archive と checksum を添付する
- archive naming、checksum、install smoke test を local と CI の release gate に組み込む
- Homebrew 定義ファイル（formula）が release artifact を参照し、`kml --version` まで検証できる状態にする
- npm / pip は独自実装ではなく、公式 binary artifact を取得して `kml` を起動する薄いラッパー（wrapper）として扱う
- README / docs / changelog が実際に公開される導線だけを公式として説明する

**Non-Goals:**

- linter rule、safe fix、formatter の挙動変更
- `kml-mcp` / `kml-mcp-remote` の新しい配布方式
- Homebrew tap の未検証 force update
- npm / pip wrapper に Markdown lint logic を実装すること
- package manager ごとに異なる CLI option や exit code を作ること

## Decisions

### D-1: primary artifact は GitHub Release binary archive にする

`v0.17.0` の配布拡張では、GitHub Release に `kml` binary archive を添付する。
Homebrew と npm / pip wrapper は、この archive と checksum を参照する。

理由は、配布チャネルごとに build logic を持つと、同じ version の `kml` が別物になるためである。
公式 binary を 1 つの release contract に集約し、各 package manager は取得と配置だけを担当する。

### D-2: artifact matrix は release runner 上で build できる platform に限定する

初期 matrix は次を対象にする。

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

各 archive は `katana-markdown-linter-vX.Y.Z-<target>.tar.gz` を基本形にし、Windows だけ `.zip` を使う。
archive には `kml` binary、`LICENSE`、短い install note を含める。

Linux の musl static build、Windows arm64、Apple notarization はこの change では要求しない。
必要になった場合は、別 change で build target と support policy を追加する。

### D-3: checksum は artifact ごとに作る

各 archive には `<archive>.sha256` を作る。
release workflow は checksum file を GitHub Release に添付し、verification は archive と checksum の対応を確認する。

Homebrew formula と wrapper は、GitHub Release の checksum を信頼元にする。
checksum が一致しない場合は install を停止する。

### D-4: Homebrew は binary formula を第一候補にする

Homebrew は `cargo install` ではなく、GitHub Release の binary archive を参照する formula を第一候補にする。
理由は、Homebrew を使う利用者に Rust toolchain を暗黙要求しないためである。

既存の `homebrew-katana` tap は KatanA Desktop 用の formula / cask を持つが、`kml` 用 formula は存在しない。
この repository 側では formula 生成・検証に必要な metadata と script を持ち、tap repository への変更は別 worktree / PR として扱う。

### D-5: npm / pip wrapper は binary artifact contract に従う

npm / pip wrapper は、公式 binary archive を取得し、checksum 検証後に `kml` を実行する薄いラッパーにする。
wrapper package は `kml` の CLI contract を変えない。

package ownership、token、名前の空き状況が未確認の場合、source と local smoke test までを作り、公開は行わない。
公開しない場合は README / docs で公式導線として宣伝しない。

### D-6: release gate は local と CI で対応を保つ

`make release-check VERSION=vX.Y.Z` は、binary package と Homebrew / wrapper smoke の local counterpart を持つ。
GitHub Release workflow は同じ script を使って archive を作り、upload する。

local で再現できない外部 publish は、release 前に metadata / checksum / install command の dry run を行う。

## Risks / Trade-offs

- Risk: platform matrix が増えるほど release 時間が伸びる
  → 初期 matrix を 4 target に限定し、追加 target は別 change にする
- Risk: Homebrew tap 更新が外部 repository の状態に影響される
  → formula 生成と検証をこの repository の責務にし、tap push は明示的な PR / merge flow に分離する
- Risk: npm / pip wrapper が新しい保守面になる
  → wrapper は binary download と process execution だけに限定し、lint logic を持たせない
- Risk: checksum や artifact 名が後から変わると package manager 側が壊れる
  → archive naming と checksum file 名を OpenSpec requirement と release test で固定する
- Risk: Windows archive と Unix archive の実行ファイル名がずれる
  → smoke test は archive 展開後の `kml` / `kml.exe` を直接実行する

## Migration Plan

1. binary artifact build script と smoke test を追加する
2. release workflow に target matrix と upload step を追加する
3. `make release-check` に binary artifact / install smoke を追加する
4. Homebrew formula 生成・検証の local script を追加する
5. npm / pip wrapper source と local smoke test を追加する
6. README、docs、CHANGELOG、OpenSpec roadmap を更新する
7. `make release-check VERSION=v0.17.0` を通す
8. release PR merge 後、通常の `make release VERSION=v0.17.0` で公開する

## Open Questions

- npm / PyPI の package name と publish credential は implementation 前に確定する必要がある
- Homebrew tap は既存 `homebrew-katana` に `kml` formula を追加するか、別 tap を作るかを implementation 前に確定する必要がある
