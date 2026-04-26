## Design

`v0.12.2` は、先に可視化してから直す。

作業を 3 つの lane に分ける。

1. CI/CD parity
2. Cache strategy
3. Linter false-positive hardening

## Current Findings

現時点の棚卸し結果:

- `test-and-build.yml` の CI matrix は `macos-latest` と `ubuntu-latest` のみで、`windows-latest` がない
- `kml` は Rust binary であり Windows でも動作対象に見えるが、release 前の Windows build / test が機械的に保証されていない
- 通常 CI は手書き `actions/cache` で `target` まで cache している
- `release.yml` と `release-preflight.yml` は `Swatinem/rust-cache` を使っており、通常 CI と cache 方針が揃っていない
- `action-smoke` と `mcp-stdio-smoke` は Unix-like shell と `bin/<name>` path を前提にしており、Windows へそのまま広げると `.exe` suffix と shell 差分で壊れやすい
- 誤検知は rule-local unit test だけでは捕まらず、複数構文が混ざる実文書 fixture が必要である
- 特に table / link / command prompt / emphasis 系は、単一行判定だけでは code block、HTML、inline code、math、reference definition と衝突する

## CI/CD Parity

通常 CI は `ubuntu-latest`、`macos-latest`、`windows-latest` の matrix にする。

ただし、全 job で同じ target を無理に実行しない。

- cross-platform required: `cargo check`, `cargo test --workspace`, `make fmt-check`
- Unix-like required: `make lint`, `make ast-lint`, `make dogfood`
- Ubuntu-only release smoke: `make action-smoke`, `make mcp-stdio-smoke`, coverage

Windows で `make` と Bash 前提の target を動かす場合は、GitHub runner 上で使用する shell を固定する。
もし shell 差分で不安定になる場合は、Windows 専用の explicit cargo command に分ける。

Release workflow は crates.io publish を含むため、引き続き Ubuntu 単独にする。
Windows は release 前の CI / preflight で検証する。

## Cache Strategy

現状は通常 CI だけ手書き `actions/cache` を使い、release / preflight は `Swatinem/rust-cache` を使っている。

`v0.12.2` では次を比較する。

- 手書き `actions/cache` のまま target cache を分離する
- 通常 CI も `Swatinem/rust-cache` に寄せる
- target cache をやめ、registry/git cache に寄せる

採用条件:

- lockfile 変更時に stale binary を再利用しない
- OS ごとの cache が混ざらない
- feature set の違いで不正な target artifact を再利用しない
- CI log で cache hit/miss を追える
- 実行時間が悪化する場合は理由を記録する

## False-Positive Hardening

誤検知は単体 rule test だけでなく、ファイル単位 fixture で固定する。

対象 context:

- fenced code block: backtick と tilde、language あり / なし、nested fence
- HTML block: anchor、image badge、inline URL、centered text
- inline code span: URL、reversed link、dollar prompt、emphasis
- math: inline `$...$`、display `$$...$$`
- table-like text: Mermaid pipes、plain text pipe、actual table
- list: nested ordered list、task list、mixed indentation
- references: used、unused、duplicate、collapsed、shortcut

単一行評価で危険な rule は、`DocumentContext` を使うか、少なくとも code block / inline span / HTML / table context を参照する。

## Self-Improvement Loop

作業中に発見した問題は `tasks.md` に即時追加する。

追加時は次の分類を付ける。

- `bug`: 実際の誤検知または誤修正
- `test-gap`: 既存実装は正しいが回帰テストが不足
- `design-debt`: rule が単一行判定に寄りすぎている
- `ci-gap`: OS / cache / workflow の検証漏れ

`v0.12.2` に含めないものは、理由と後続 change を記録する。

## Risks

- Windows CI は shell / path / executable suffix の差分で不安定になりやすい
- cache を変えると CI 実行時間が一時的に悪化する可能性がある
- 誤検知 fixture を増やすだけで rule 設計を直さないと、同じ問題が別 rule で再発する

## Mitigation

- Windows ではまず cargo command ベースで通し、Makefile target の移植は段階的に行う
- cache 変更前後の CI duration を PR に記録する
- 誤検知修正は必ずファイル単位 fixture と rule-local fixture の両方に落とす
